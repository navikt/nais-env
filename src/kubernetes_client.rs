use k8s_openapi::api::core::v1::Secret;
use kube::{Api, Client, Config};
use std::{collections::BTreeMap, str};

pub struct KubernetesClient {
    client: Client,
    deployment: String,
    namespace: String,
    context: String,
}

impl KubernetesClient {
    /// Creates a new Kubernetes client instance configured for a specific namespace, deployment, and context.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The Kubernetes namespace to operate within
    /// * `deployment` - The name of the deployment to target
    /// * `context` - The Kubernetes context to use (e.g., 'nais-dev' or 'dev-fss')
    ///
    /// # Returns
    ///
    /// A configured `KubernetesClient` instance ready to interact with the specified namespace and deployment.
    ///
    /// # Errors
    ///
    /// Returns a `kube::Error` if the Kubernetes configuration cannot be loaded or if the client
    /// cannot be created from the configuration.
    pub async fn new(
        namespace: String,
        deployment: String,
        context: String,
    ) -> Result<Self, kube::Error> {
        let mut config = Config::from_kubeconfig(&kube::config::KubeConfigOptions {
            context: Some(context.clone()),
            ..Default::default()
        })
        .await
        .expect("Failed to load kubernetes config");

        config.default_namespace = namespace.clone();

        let client = Client::try_from(config)?;

        Ok(Self {
            client,
            deployment,
            namespace,
            context,
        })
    }

    /// Retrieves a Kubernetes secret and converts its data into a key-value map.
    ///
    /// # Arguments
    ///
    /// * `secret_name` - The name of the secret to retrieve
    ///
    /// # Returns
    ///
    /// A map containing all key-value pairs from the secret's data field.
    /// If the secret exists but has no data, an empty map is returned.
    ///
    /// # Errors
    ///
    /// Returns an error if the secret cannot be retrieved from the Kubernetes API
    /// or if there's an issue parsing the secret data.
    pub async fn get_secret(
        &self,
        secret_name: &str,
    ) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
        let secret: Secret = Api::default_namespaced(self.client.clone())
            .get(secret_name)
            .await?;

        let env_vars: BTreeMap<String, String> = if let Some(data) = &secret.data {
            data.iter()
                .map(|(key, value)| {
                    (
                        key.clone(),
                        String::from_utf8(value.0.clone()).unwrap_or_default(),
                    )
                })
                .collect()
        } else {
            BTreeMap::new()
        };

        Ok(env_vars)
    }

    /// This method fetches the deployment configuration from Kubernetes and extracts
    /// secret names that are referenced in the `envFrom` section of any container
    /// in the deployment. These secrets typically contain environment variables that
    /// should be loaded into the container.
    ///
    /// # Returns
    ///
    /// A sorted vector of unique secret names referenced by the deployment.
    ///
    /// # Errors
    ///
    /// Returns an error if fetching the deployment information fails or if there's
    /// an issue accessing the Kubernetes API.
    pub async fn get_env_from_secrets(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let api =
            Api::<k8s_openapi::api::apps::v1::Deployment>::default_namespaced(self.client.clone());

        let deployment = api.get(&self.deployment).await.map_err(|e| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Failed to get deployment '{}' in namespace '{}' with context '{}': {}",
                    self.deployment, self.namespace, self.context, e
                ),
            )) as Box<dyn std::error::Error>
        })?;

        let mut secret_names = Vec::new();

        if let Some(spec) = deployment.spec {
            let template = spec.template;
            if let Some(pod_spec) = template.spec {
                for container in pod_spec.containers {
                    if let Some(env_from) = container.env_from {
                        for env_source in env_from {
                            if let Some(secret_ref) = env_source.secret_ref {
                                secret_names.push(secret_ref.name);
                            }
                        }
                    }
                }
            }
        }

        // Remove duplicates
        secret_names.sort();
        secret_names.dedup();

        Ok(secret_names)
    }
}
