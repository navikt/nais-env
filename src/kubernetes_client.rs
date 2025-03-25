use k8s_openapi::api::{apps::v1::Deployment, core::v1::Secret};
use kube::{Api, Client, Config};
use std::str;

use crate::env_var::EnvVar;

pub struct KubernetesClient {
    client: Client,
    deployment: String,
}

impl KubernetesClient {
    pub async fn new(namespace: String, deployment: String) -> Result<Self, kube::Error> {
        let mut config = Config::infer()
            .await
            .expect("Failed to find kubernetes config");
        config.default_namespace = namespace.clone();

        let client = Client::try_from(config).expect("Could not set up kubernetes client");

        Ok(Self { client, deployment })
    }

    pub async fn get_deployment(
        &self,
        deployment: &str,
    ) -> Result<Deployment, Box<dyn std::error::Error>> {
        let deployment: Deployment = Api::default_namespaced(self.client.clone())
            .get(deployment)
            .await?;

        Ok(deployment)
    }

    pub async fn get_secret(
        &self,
        secret_name: &str,
    ) -> Result<Vec<EnvVar>, Box<dyn std::error::Error>> {
        let secret: Secret = Api::default_namespaced(self.client.clone())
            .get(secret_name)
            .await?;

        // Parse secret.data into Vector of EnvVar
        let env_vars: Vec<EnvVar> = if let Some(data) = &secret.data {
            data.iter()
                .map(|(key, value)| EnvVar {
                    name: key.clone(),
                    value: String::from_utf8(value.0.clone()).unwrap_or_default(),
                })
                .collect()
        } else {
            Vec::new()
        };

        Ok(env_vars)
    }

    pub async fn get_secrets(&self) -> Result<Vec<EnvVar>, Box<dyn std::error::Error>> {
        let deployment = self.get_deployment(self.deployment.as_ref()).await?;

        let containers = match &deployment.spec {
            Some(spec) => match &spec.template.spec {
                Some(pod_spec) => &pod_spec.containers,
                None => return Err("Pod spec not found".into()),
            },
            None => return Err("Deployment spec not found".into()),
        };

        let container = containers
            .iter()
            .find(|container| container.name == self.deployment)
            .ok_or("Container not found")?;

        let env_from = match &container.env_from {
            Some(env_from) => env_from,
            None => return Ok(Vec::new()),
        };

        let secrets_to_fetch: Vec<String> = env_from
            .iter()
            .map(|env| {
                env.secret_ref
                    .as_ref()
                    .map(|secret| secret.name.clone())
                    .unwrap_or_default()
            })
            .collect();

        let mut env_vars = Vec::new();
        for secret_name in secrets_to_fetch {
            let mut secret_env_vars = self.get_secret(&secret_name).await?;
            env_vars.append(&mut secret_env_vars);
        }

        Ok(env_vars)
    }
}
