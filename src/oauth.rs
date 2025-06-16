use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenResponse as OAuth2TokenResponse, TokenUrl,
};
use oauth2::{StandardTokenResponse, basic::BasicClient, reqwest::async_http_client};
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use url::Url;

/// OAuth 2.0 client implementation
pub struct OAuthClient {
    /// OAuth2 client from oauth2 crate
    oauth_client: BasicClient,
    /// HTTP client for making requests
    http_client: reqwest::Client,
}

impl OAuthClient {
    /// Create a new OAuth client with the given parameters
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        auth_base_url: String,
    ) -> Self {
        let auth_url = format!("{}/authorize", auth_base_url);
        let token_url = format!("{}/token", auth_base_url);

        let oauth_client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(auth_url).expect("Invalid authorization URL"),
            Some(TokenUrl::new(token_url).expect("Invalid token URL")),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri).expect("Invalid redirect URI"));

        Self {
            oauth_client,
            http_client: reqwest::Client::new(),
        }
    }

    /// Create a new OAuth client from environment variables and handle authentication
    /// Returns the access token directly
    pub async fn new_from_env(
        env: &std::collections::BTreeMap<String, String>,
        scope: String,
    ) -> Result<String, OAuthError> {
        // Extract required variables from the environment
        let client_id = env
            .get("AZURE_APP_CLIENT_ID")
            .ok_or_else(|| OAuthError::InvalidRequest("Missing AZURE_APP_CLIENT_ID".to_string()))?
            .clone();

        let client_secret = env
            .get("AZURE_APP_CLIENT_SECRET")
            .ok_or_else(|| {
                OAuthError::InvalidRequest("Missing AZURE_APP_CLIENT_SECRET".to_string())
            })?
            .clone();

        // Get tenant ID (required)
        let tenant_id = env
            .get("AZURE_APP_TENANT_ID")
            .ok_or_else(|| OAuthError::InvalidRequest("Missing AZURE_APP_TENANT_ID".to_string()))?;

        // Construct base URL from tenant ID
        let auth_base_url = format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0",
            tenant_id
        );

        let port = 5556;
        // Create redirect URI using localhost and the port
        let redirect_uri = format!("http://localhost:{}/auth/callback", port);

        // Get scopes from environment
        let scopes = vec![scope];

        // Create the OAuth client
        let client = Self::new(
            client_id.clone(),
            client_secret.clone(),
            redirect_uri.clone(),
            auth_base_url.clone(),
        );

        // Check if we already have an authorization code in the environment
        if let Some(auth_code) = env.get("OAUTH_AUTHORIZATION_CODE") {
            // Use the existing code
            let token_response = client.exchange_code_for_token(auth_code.clone()).await?;
            return Ok(token_response.access_token);
        }

        // Generate PKCE challenge
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        // Get the authorization URL
        let (auth_url, csrf_token) = client.get_authorization_url_pkce(scopes, pkce_challenge);

        println!("Authorization URL: {}", auth_url);

        // Start a local web server to receive the redirect
        let received_code = Self::start_redirect_server(port, csrf_token.secret())?;

        // Now exchange the code for a token
        let token_response = client
            .exchange_code_for_token_pkce(received_code, pkce_verifier)
            .await?;

        Ok(token_response.access_token)
    }

    /// Start a local web server to receive the OAuth redirect and authorization code
    fn start_redirect_server(port: u16, csrf_state: &str) -> Result<String, OAuthError> {
        println!("Starting OAuth redirect server on port {}...", port);

        // Create socket address for the server
        let socket_addr = SocketAddr::from(([127, 0, 0, 1], port));

        // Create a TCP listener
        let listener = TcpListener::bind(socket_addr).map_err(|e| {
            OAuthError::ServerError(format!("Failed to start redirect server: {}", e))
        })?;

        println!(
            "Please open your browser and visit the authorization URL to authorize the application."
        );
        println!("After authorizing, you'll be redirected back to localhost.");

        // Create a shared variable to store the received code
        let received_code = Arc::new(Mutex::new(None));
        let received_code_clone = received_code.clone();

        // Need to clone the csrf_state to move into thread
        let csrf_state_owned = csrf_state.to_string();

        // Create a thread to handle the redirect
        let handle = std::thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        // Read the request
                        let mut reader = BufReader::new(&stream);
                        let mut request_line = String::new();
                        reader.read_line(&mut request_line).unwrap_or_default();

                        // Check if this is a GET request to our callback path
                        if request_line.starts_with("GET /auth/callback") {
                            // Extract code and state from the URL
                            let query_params = request_line.split('?').nth(1);

                            if let Some(query) = query_params {
                                // Parse the query parameters
                                let params: Vec<(String, String)> = query
                                    .split('&')
                                    .map(|kv| {
                                        let mut parts = kv.splitn(2, '=');
                                        let key = parts.next().unwrap_or_default().to_string();
                                        let value = parts
                                            .next()
                                            .unwrap_or_default()
                                            .split(' ')
                                            .next()
                                            .unwrap_or_default()
                                            .to_string();
                                        (key, value)
                                    })
                                    .collect();

                                // Look for code and state
                                let mut code = None;
                                let mut state = None;

                                for (key, value) in params {
                                    if key == "code" {
                                        code = Some(value);
                                    } else if key == "state" {
                                        state = Some(value);
                                    }
                                }

                                // Verify state and store code if valid
                                if let (Some(received_code), Some(received_state)) = (code, state) {
                                    if received_state == csrf_state_owned {
                                        *received_code_clone.lock().unwrap() = Some(received_code);

                                        // Send success response
                                        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
                                        <html><body><h1>Authentication Successful</h1>\
                                        <p>You can now close this window and return to the application.</p></body></html>";
                                        stream.write_all(response.as_bytes()).unwrap_or_default();
                                        break;
                                    } else {
                                        // Send error response - invalid state
                                        let response = "HTTP/1.1 400 Bad Request\r\nContent-Type: text/html\r\n\r\n\
                                        <html><body><h1>Authentication Failed</h1>\
                                        <p>Invalid state parameter. This might be a CSRF attack.</p></body></html>";
                                        stream.write_all(response.as_bytes()).unwrap_or_default();
                                    }
                                } else {
                                    // Send error response - missing parameters
                                    let response = "HTTP/1.1 400 Bad Request\r\nContent-Type: text/html\r\n\r\n\
                                    <html><body><h1>Authentication Failed</h1>\
                                    <p>Missing code or state parameter in the redirect.</p></body></html>";
                                    stream.write_all(response.as_bytes()).unwrap_or_default();
                                }
                            }
                        } else {
                            // Send a simple response for other requests
                            let response = "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\nNot Found";
                            stream.write_all(response.as_bytes()).unwrap_or_default();
                        }
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection: {}", e);
                    }
                }
            }
        });

        // Wait a moment and then open the browser to the auth URL
        println!("Press Ctrl+C to cancel the authentication process.");

        // Wait for the thread to complete or timeout
        handle.join().unwrap_or_default();

        // Get the received code
        let code = received_code.lock().unwrap().clone();

        match code {
            Some(auth_code) => Ok(auth_code),
            None => Err(OAuthError::ServerError(
                "Failed to receive authorization code".to_string(),
            )),
        }
    }

    /// Generate the authorization URL for initiating the OAuth flow with PKCE
    pub fn get_authorization_url_pkce(
        &self,
        scopes: Vec<String>,
        pkce_challenge: PkceCodeChallenge,
    ) -> (Url, CsrfToken) {
        let mut auth_request = self
            .oauth_client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_challenge);

        // Add requested scopes
        for scope in scopes {
            auth_request = auth_request.add_scope(Scope::new(scope));
        }

        auth_request.url()
    }

    /// Exchange an authorization code for an access token
    pub async fn exchange_code_for_token(&self, code: String) -> Result<TokenResponse, OAuthError> {
        let token_result = self
            .oauth_client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .map_err(OAuthError::OAuth2Error)?;

        Ok(TokenResponse::from_oauth2_token(token_result))
    }

    /// Exchange an authorization code for an access token with PKCE verification
    pub async fn exchange_code_for_token_pkce(
        &self,
        code: String,
        pkce_verifier: oauth2::PkceCodeVerifier,
    ) -> Result<TokenResponse, OAuthError> {
        let token_result = self
            .oauth_client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await
            .map_err(OAuthError::OAuth2Error)?;

        Ok(TokenResponse::from_oauth2_token(token_result))
    }
}

/// Represents the response from a token request
#[derive(Debug, Clone)]
pub struct TokenResponse {
    /// The access token issued by the authorization server
    pub access_token: String,
    /// The type of token, typically "Bearer"
    pub token_type: String,
    /// The lifetime of the access token in seconds
    pub expires_in: u64,
    /// Token used to obtain a new access token when the current one expires
    pub refresh_token: Option<String>,
    /// Space-separated list of scopes granted to the client
    pub scope: Option<String>,
}

impl TokenResponse {
    /// Convert oauth2 crate's token response to our TokenResponse
    fn from_oauth2_token(
        token: StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
    ) -> Self {
        Self {
            access_token: token.access_token().secret().clone(),
            token_type: token.token_type().as_ref().to_string(),
            expires_in: token.expires_in().unwrap_or_default().as_secs(),
            refresh_token: token.refresh_token().map(|rt| rt.secret().clone()),
            scope: token.scopes().map(|scopes| {
                scopes
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            }),
        }
    }
}

/// Custom error type for OAuth operations
#[derive(Debug, Error)]
pub enum OAuthError {
    /// Network-related errors
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// OAuth2 library errors
    #[error("OAuth2 error: {0}")]
    OAuth2Error(
        #[from]
        oauth2::RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
        >,
    ),

    /// Server errors (for redirect server)
    #[error("Server error: {0}")]
    ServerError(String),

    /// Invalid or missing parameters
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}
