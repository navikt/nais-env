use std::{
    env,
    process::{Command, Stdio},
};
mod kubernetes_client;
mod nais;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let nais_config_path = if args.len() > 1 {
        &args[1]
    } else {
        panic!("Missing config file")
    };

    let nais_config = nais::NaisConfigLoader::new(nais_config_path.clone()).expect("data");

    let current_namespace = nais_config.get_namespace();
    let current_deployment = nais_config.get_deployment();

    let kubernetes_client = kubernetes_client::KubernetesClient::new(
        current_namespace.to_string(),
        current_deployment.to_string(),
    )
    .await
    .expect("Failed to create Kubernetes client");

    let mut env_vars = nais_config.get_env_vars();

    match kubernetes_client
        .get_deployment(nais_config.get_deployment())
        .await
    {
        Ok(_) => println!("Successfully fetched deployment"),
        Err(e) => panic!("Failed to fetch deployment: {}", e),
    }

    let secret_keys = nais_config.get_env_var_from_secret_keys();
    println!("Secret keys to fetch: {:?}", secret_keys);

    let mut collected_secrets = Vec::new();
    for secret_name in secret_keys {
        match kubernetes_client.get_secret(&secret_name).await {
            Ok(secrets) => collected_secrets.extend(secrets),
            Err(e) => eprintln!("Failed to fetch secret {}: {}", secret_name, e),
        }
    }

    for (key, value) in &collected_secrets {
        env_vars.insert(key.clone(), value.clone());
    }

    for (key, value) in &env_vars {
        println!("VAR {} = VALUE {}", key, value);
    }

    let shell_binary = env::var("SHELL").unwrap();
    println!("Shell binary: {}", shell_binary);

    let mut command = Command::new("zsh");

    for (key, value) in &env_vars {
        command.env(key, value);
    }

    let child = Command::new("sh")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .envs(&env_vars)
        .spawn()
        .unwrap();
}
