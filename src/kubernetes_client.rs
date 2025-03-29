use k8s_openapi::api::core::v1::Secret;
use kube::{Api, Client, Config};
use std::{collections::HashMap, str};

pub struct KubernetesClient {
    client: Client,
}

impl KubernetesClient {
    pub async fn new(namespace: String) -> Result<Self, kube::Error> {
        let mut config = Config::infer()
            .await
            .expect("Failed to find kubernetes config");
        config.default_namespace = namespace.clone();

        let client = Client::try_from(config).unwrap();

        Ok(Self { client })
    }

    pub async fn get_secret(
        &self,
        secret_name: &str,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let secret: Secret = Api::default_namespaced(self.client.clone())
            .get(secret_name)
            .await?;

        let env_vars: HashMap<String, String> = if let Some(data) = &secret.data {
            data.iter()
                .map(|(key, value)| {
                    (
                        key.clone(),
                        String::from_utf8(value.0.clone()).unwrap_or_default(),
                    )
                })
                .collect()
        } else {
            HashMap::new()
        };

        Ok(env_vars)
    }
}
