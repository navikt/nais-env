use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::env_var::EnvVar;

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
    pub fn new(config_path: String) -> Result<Self, Box<dyn std::error::Error>> {
        let content = match std::fs::read_to_string(&config_path) {
            Ok(content) => content,
            Err(e) => {
                return Err(format!("Failed to read config file {}: {}", config_path, e).into());
            }
        };

        // Check if kind is "Application" before parsing and panic if not
        if !content.contains("kind: \"Application\"") {
            panic!("Expected kind: Application");
        }

        let config: NaisConfig = match serde_yaml::from_str(&content) {
            Ok(config) => config,
            Err(e) => panic!("Failed to parse config as YAML: {}", e),
        };

        Ok(NaisConfigLoader { config })
    }

    pub fn get_namespace(&self) -> &str {
        &self.config.metadata.namespace
    }

    pub fn get_deployment(&self) -> &str {
        &self.config.metadata.name
    }

    pub fn get_env_vars(&self) -> Vec<EnvVar> {
        let mut env_vars = Vec::new();
        if let Some(env) = &self.config.spec.env {
            for e in env {
                env_vars.push(EnvVar {
                    name: e.name.clone(),
                    value: e.value.clone().unwrap_or_default(),
                });
            }
        }
        env_vars
    }

    pub fn get_env_var_from_secret_keys(&self) -> Vec<String> {
        let secret_keys = self
            .config
            .spec
            .envFrom
            .as_ref()
            .map(|env_from| {
                env_from
                    .iter()
                    .filter_map(|e| e.secret.as_ref().map(|secret| secret.clone()))
                    .collect()
            })
            .expect("Could not get secrets from env");

        secret_keys
    }
}
