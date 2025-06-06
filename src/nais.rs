use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct NaisConfig {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: Metadata,
    pub spec: Spec,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(default)]
    pub labels: HashMap<String, String>,
    pub name: String,
    pub namespace: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Spec {
    #[serde(default)]
    pub accessPolicy: Option<AccessPolicy>,
    #[serde(default)]
    pub azure: Option<Azure>,
    #[serde(default)]
    pub env: Option<Vec<NaisEnvVar>>,
    #[serde(default)]
    pub envFrom: Option<Vec<EnvFrom>>,
    #[serde(default)]
    pub filesFrom: Option<Vec<FilesFrom>>,
    #[serde(default)]
    pub idporten: Option<Idporten>,
    pub image: String,
    #[serde(default)]
    pub ingresses: Option<Vec<String>>,
    #[serde(default)]
    pub maskinporten: Option<Maskinporten>,
    #[serde(default)]
    pub tokenx: Option<Tokenx>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessPolicy {
    #[serde(default)]
    pub inbound: Option<InboundPolicy>,
    #[serde(default)]
    pub outbound: Option<OutboundPolicy>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InboundPolicy {
    #[serde(default)]
    pub rules: Option<Vec<AccessRule>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutboundPolicy {
    #[serde(default)]
    pub external: Option<Vec<ExternalRule>>,
    #[serde(default)]
    pub rules: Option<Vec<AccessRule>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessRule {
    pub application: String,
    #[serde(default)]
    pub namespace: Option<String>,
    #[serde(default)]
    pub cluster: Option<String>,
    #[serde(default)]
    pub permissions: Option<Permissions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permissions {
    #[serde(default)]
    pub roles: Option<Vec<String>>,
    #[serde(default)]
    pub scopes: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalRule {
    #[serde(default)]
    pub host: Option<String>,
    #[serde(default)]
    pub ipv4: Option<String>,
    #[serde(default)]
    pub ports: Option<Vec<Port>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Port {
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Azure {
    #[serde(default)]
    pub application: Option<AzureApplication>,
    #[serde(default)]
    pub sidecar: Option<AzureSidecar>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AzureApplication {
    #[serde(default)]
    pub allowAllUsers: Option<bool>,
    #[serde(default)]
    pub claims: Option<AzureClaims>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub tenant: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureClaims {
    #[serde(default)]
    pub groups: Option<Vec<AzureGroup>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureGroup {
    pub id: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AzureSidecar {
    #[serde(default)]
    pub autoLogin: Option<bool>,
    #[serde(default)]
    pub autoLoginIgnorePaths: Option<Vec<String>>,
    #[serde(default)]
    pub enabled: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct NaisEnvVar {
    pub name: String,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub valueFrom: Option<ValueFrom>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ValueFrom {
    #[serde(default)]
    pub fieldRef: Option<FieldRef>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct FieldRef {
    pub fieldPath: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvFrom {
    #[serde(default)]
    pub configmap: Option<String>,
    #[serde(default)]
    pub secret: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct FilesFrom {
    #[serde(default)]
    pub configmap: Option<String>,
    #[serde(default)]
    pub secret: Option<String>,
    #[serde(default)]
    pub emptyDir: Option<EmptyDir>,
    #[serde(default)]
    pub persistentVolumeClaim: Option<String>,
    pub mountPath: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyDir {
    #[serde(default)]
    pub medium: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Idporten {
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub sidecar: Option<IdportenSidecar>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct IdportenSidecar {
    #[serde(default)]
    pub autoLogin: Option<bool>,
    #[serde(default)]
    pub autoLoginIgnorePaths: Option<Vec<String>>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub level: Option<String>,
    #[serde(default)]
    pub locale: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Maskinporten {
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub scopes: Option<MaskinportenScopes>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaskinportenScopes {
    #[serde(default)]
    pub consumes: Option<Vec<MaskinportenConsume>>,
    #[serde(default)]
    pub exposes: Option<Vec<MaskinportenExpose>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaskinportenConsume {
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct MaskinportenExpose {
    #[serde(default)]
    pub accessibleForAll: Option<bool>,
    #[serde(default)]
    pub allowedIntegrations: Option<Vec<String>>,
    #[serde(default)]
    pub atMaxAge: Option<u32>,
    #[serde(default)]
    pub consumers: Option<Vec<MaskinportenConsumer>>,
    #[serde(default)]
    pub delegationSource: Option<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
    pub name: String,
    pub product: String,
    #[serde(default)]
    pub separator: Option<String>,
    #[serde(default)]
    pub visibility: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaskinportenConsumer {
    pub name: String,
    pub orgno: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tokenx {
    #[serde(default)]
    pub enabled: Option<bool>,
}

pub struct NaisConfigLoader {
    config: NaisConfig,
}

impl NaisConfigLoader {
    /// Creates a new `NaisConfigLoader` instance from a configuration file.
    ///
    /// This function reads the NAIS configuration from the specified file path,
    /// validates that it contains an "Application" kind, and parses it into a
    /// structured representation.
    ///
    /// # Arguments
    /// * `config_path` - The path to the NAIS configuration file
    ///
    /// # Returns
    /// A `Result` containing either the constructed `NaisConfigLoader` or an error
    /// if the file cannot be read or parsed.
    ///
    /// # Errors
    /// This function will return an error if:
    /// * The configuration file cannot be read
    /// * The configuration does not contain "kind: \"Application\""
    /// * The YAML cannot be parsed into the expected structure
    ///
    /// # Example
    /// ```
    /// let config_loader = NaisConfigLoader::new("nais.yaml".to_string())?;
    /// ```
    pub fn new(config_path: String) -> Result<Self, Box<dyn std::error::Error>> {
        let content = match std::fs::read_to_string(&config_path) {
            Ok(content) => content,
            Err(e) => {
                return Err(format!("Failed to read config file {}: {}", config_path, e).into());
            }
        };

        if !content.contains("kind: \"Application\"")
            && !content.contains("kind: \'Application\'")
            && !content.contains("kind: Application")
        {
            return Err("Expected kind: Application".into());
        }

        let config: NaisConfig = match serde_yaml::from_str(&content) {
            Ok(config) => config,
            Err(e) => panic!("Failed to parse config as YAML: {}", e),
        };

        Ok(NaisConfigLoader { config })
    }

    /// Creates a new `NaisConfigLoader` instance from a configuration file with variable substitution,
    /// and returns both the loader and the processed template content.
    ///
    /// This function reads the NAIS configuration from the specified file path,
    /// substitutes variables, parses it into a structured representation,
    /// and returns the processed template string as well.
    ///
    /// # Arguments
    /// * `config_path` - The path to the NAIS configuration file
    /// * `variables` - A YAML structure containing variables for substitution
    ///
    /// # Returns
    /// A `Result` containing either a tuple of the constructed `NaisConfigLoader` and the
    /// processed template content, or an error if the file cannot be read or parsed.
    ///
    /// # Example
    /// ```
    /// let variables = yaml_vars::parse_variables_file("vars.yaml")?;
    /// let (config_loader, processed_template) = NaisConfigLoader::new_with_variables_and_template(
    ///     "nais.yaml".to_string(),
    ///     variables
    /// )?;
    /// ```
    pub fn new_with_variables_and_template(
        config_path: String,
        variables: serde_yaml::Value,
    ) -> Result<(Self, String), Box<dyn std::error::Error>> {
        let content = match std::fs::read_to_string(&config_path) {
            Ok(content) => content,
            Err(e) => {
                return Err(format!("Failed to read config file {}: {}", config_path, e).into());
            }
        };

        // Substitute variables in the content using template syntax
        let processed_content = crate::yaml_vars::substitute_variables(&content, &variables);

        if !processed_content.contains("kind: \"Application\"")
            && !processed_content.contains("kind: \'Application\'")
            && !processed_content.contains("kind: Application")
        {
            return Err("Expected kind: Application".into());
        }

        let config: NaisConfig = match serde_yaml::from_str(&processed_content) {
            Ok(config) => config,
            Err(e) => panic!("Failed to parse config as YAML: {}", e),
        };

        Ok((NaisConfigLoader { config }, processed_content))
    }

    /// Retrieves the namespace from the NAIS configuration.
    ///
    /// This method returns the namespace specified in the metadata section
    /// of the NAIS configuration file.
    ///
    /// # Returns
    /// A `String` containing the namespace.
    ///
    /// # Example
    /// ```
    /// let config_loader = NaisConfigLoader::new("nais.yaml".to_string()).unwrap();
    /// let namespace = config_loader.get_namespace();
    /// println!("Namespace: {}", namespace);
    /// ```
    pub fn get_namespace(&self) -> String {
        self.config.metadata.namespace.clone()
    }

    /// Retrieves the deployment name from the NAIS configuration.
    ///
    /// This method returns the name of the application as defined in the metadata
    /// section of the NAIS configuration file.
    ///
    /// # Returns
    /// A `String` containing the deployment name.
    ///
    /// # Example
    /// ```
    /// let config_loader = NaisConfigLoader::new("nais.yaml".to_string()).unwrap();
    /// let deployment_name = config_loader.get_deployment();
    /// println!("Deployment name: {}", deployment_name);
    /// ```
    pub fn get_deployment(&self) -> String {
        self.config.metadata.name.clone()
    }

    /// Retrieves all environment variables defined in the NAIS configuration file.
    ///
    /// # Returns
    /// A sorted `BTreeMap` where keys are environment variable names and values are their corresponding values.
    /// If an environment variable is defined without a value in the config, an empty string is used as default.
    ///
    /// # Example
    /// ```
    /// let config_loader = NaisConfigLoader::new("nais.yaml".to_string()).unwrap();
    /// let env_vars = config_loader.get_env_vars();
    /// // Use the environment variables
    /// ```
    pub fn get_env_vars(&self) -> std::collections::BTreeMap<String, String> {
        let mut env_vars = std::collections::BTreeMap::new();
        if let Some(env) = &self.config.spec.env {
            for e in env {
                env_vars.insert(e.name.clone(), e.value.clone().unwrap_or_default());
            }
        }
        env_vars
    }
}
