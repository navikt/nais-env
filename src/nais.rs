use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Spec {
    #[serde(default)]
    pub accessPolicy: Option<AccessPolicy>,
    #[serde(default)]
    pub azure: Option<Azure>,
    #[serde(default)]
    pub env: Option<Vec<EnvVar>>,
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
    pub login: Option<Login>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureSidecar {
    #[serde(default)]
    pub autoLogin: Option<bool>,
    #[serde(default)]
    pub autoLoginIgnorePaths: Option<Vec<String>>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub resources: Option<Resources>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvVar {
    pub name: String,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub valueFrom: Option<ValueFrom>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueFrom {
    #[serde(default)]
    pub fieldRef: Option<FieldRef>,
}

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
pub struct Frontend {
    #[serde(default)]
    pub generatedConfig: Option<GeneratedConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedConfig {
    pub mountPath: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LifecycleCondition {
    #[serde(default)]
    pub age: Option<u32>,
    #[serde(default)]
    pub createdBefore: Option<String>,
    #[serde(default)]
    pub numNewerVersions: Option<u32>,
    #[serde(default)]
    pub withState: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GcpPermission {
    pub resource: GcpResource,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GcpResource {
    pub apiVersion: String,
    pub kind: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SqlInstance {
    #[serde(default)]
    pub autoBackupHour: Option<u32>,
    #[serde(default)]
    pub cascadingDelete: Option<bool>,
    #[serde(default)]
    pub collation: Option<String>,
    #[serde(default)]
    pub databases: Option<Vec<SqlDatabase>>,
    #[serde(default)]
    pub diskAutoresize: Option<bool>,
    #[serde(default)]
    pub diskAutoresizeLimit: Option<u32>,
    #[serde(default)]
    pub diskSize: Option<u32>,
    #[serde(default)]
    pub diskType: Option<String>,
    #[serde(default)]
    pub flags: Option<Vec<SqlFlag>>,
    #[serde(default)]
    pub highAvailability: Option<bool>,
    #[serde(default)]
    pub insights: Option<SqlInsights>,
    #[serde(default)]
    pub maintenance: Option<SqlMaintenance>,
    pub name: String,
    #[serde(default)]
    pub pointInTimeRecovery: Option<bool>,
    #[serde(default)]
    pub retainedBackups: Option<u32>,
    #[serde(default)]
    pub tier: Option<String>,
    #[serde(default)]
    pub transactionLogRetentionDays: Option<u32>,
    #[serde(default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SqlDatabase {
    #[serde(default)]
    pub envVarPrefix: Option<String>,
    pub name: String,
    #[serde(default)]
    pub users: Option<Vec<SqlUser>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SqlUser {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SqlFlag {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SqlInsights {
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub queryStringLength: Option<u32>,
    #[serde(default)]
    pub recordApplicationTags: Option<bool>,
    #[serde(default)]
    pub recordClientAddress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SqlMaintenance {
    pub day: u32,
    pub hour: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Idporten {
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub sidecar: Option<IdportenSidecar>,
}

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
    #[serde(default)]
    pub resources: Option<Resources>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Influx {
    pub instance: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kafka {
    pub pool: String,
    #[serde(default)]
    pub streams: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Probe {
    #[serde(default)]
    pub failureThreshold: Option<u32>,
    #[serde(default)]
    pub initialDelay: Option<u32>,
    pub path: String,
    #[serde(default)]
    pub periodSeconds: Option<u32>,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default)]
    pub timeout: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    #[serde(default)]
    pub enforce: Option<Enforce>,
    #[serde(default)]
    pub provider: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Enforce {
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub excludePaths: Option<Vec<String>>,
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
pub struct Observability {
    #[serde(default)]
    pub autoInstrumentation: Option<AutoInstrumentation>,
    #[serde(default)]
    pub logging: Option<Logging>,
    #[serde(default)]
    pub tracing: Option<Tracing>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoInstrumentation {
    #[serde(default)]
    pub destinations: Option<Vec<Destination>>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub runtime: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Destination {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Logging {
    #[serde(default)]
    pub destinations: Option<Vec<Destination>>,
    #[serde(default)]
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tracing {
    #[serde(default)]
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenSearch {
    pub access: String,
    pub instance: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreStopHook {
    #[serde(default)]
    pub exec: Option<Exec>,
    #[serde(default)]
    pub http: Option<Http>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exec {
    pub command: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Http {
    pub path: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prometheus {
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub port: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Redirect {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tokenx {
    #[serde(default)]
    pub enabled: Option<bool>,
}

pub fn load_config(config_path: &String) -> Result<NaisConfig, Box<dyn std::error::Error>> {
    let content = match std::fs::read_to_string(&config_path) {
        Ok(content) => content,
        Err(e) => return Err(format!("Failed to read config file {}: {}", config_path, e).into()),
    };

    // Check if kind is "Application" before parsing
    if !content.contains("kind: \"Application\"") {
        return Err("Expected kind: Application".into());
    }

    let config: NaisConfig = match serde_yaml::from_str(&content) {
        Ok(config) => config,
        Err(e) => return Err(format!("Failed to parse config as YAML: {}", e).into()),
    };

    Ok(config)
}
