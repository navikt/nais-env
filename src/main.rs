use std::{env, process::Command};

use clap::Parser;
mod env_file;
mod git;
mod kubernetes_client;
mod nais;

/// Set up configuration from Nais locally
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Save Nais configuration to file
    #[arg(short, long)]
    file: Option<String>,

    /// Files with environment variables that override the ones from Kubernetes
    #[arg(short, long)]
    overrides: Option<Vec<String>>,

    /// Path to nais.yaml
    #[arg(short, long)]
    config: String,

    /// Print secrets
    #[arg(short, long)]
    print: bool,

    /// Spawn shell with secrets as enviroment variables
    #[arg(short, long)]
    shell: bool,

    /// Clear all files added by nais-env (must be in git repository)
    #[arg(long)]
    clear_files: bool,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.clear_files {
        if let Err(e) = env_file::clear_env_files() {
            eprintln!("Failed to clear env files: {}", e);
            return Err(e);
        }
        println!("Successfully cleared environment files");
        return Ok(());
    }

    let file_path = args.file;
    let config_file = args.config;

    let overrides = if let Some(override_files) = args.overrides {
        env_file::parse_multiple_env_files(override_files)?
    } else {
        std::collections::BTreeMap::new()
    };

    let nais_config =
        nais::NaisConfigLoader::new(config_file.clone()).expect("Could not load config file");

    let kubernetes_client = kubernetes_client::KubernetesClient::new(
        nais_config.get_namespace(),
        nais_config.get_deployment(),
    )
    .await
    .expect("Failed to create Kubernetes client");

    let secret_keys = kubernetes_client
        .get_env_from_secrets()
        .await
        .expect("Could not get secrets from kubernetes");

    let mut collected_secrets = std::collections::BTreeMap::new();
    for secret_name in secret_keys {
        match kubernetes_client.get_secret(&secret_name).await {
            Ok(secrets) => {
                for (key, value) in secrets {
                    collected_secrets.insert(key, value);
                }
            }
            Err(e) => eprintln!("Failed to fetch secret {}: {}", secret_name, e),
        }
    }

    let nais_config_env_vars = nais_config.get_env_vars();

    // Combine env_vars and secrets into a sorted map
    let all_env_vars: std::collections::BTreeMap<String, String> = collected_secrets
        .into_iter()
        .chain(nais_config_env_vars)
        .chain(overrides)
        .collect();

    if let Some(file) = file_path {
        match env_file::save_env_vars_to_file(&file, &all_env_vars) {
            Ok(_) => println!("Successfully saved environment variables to file: {}", file),
            Err(e) => eprintln!("Failed to save environment variables to file: {}", e),
        }
    }

    if args.print {
        println!("Environment Variables:");
        for (key, value) in &all_env_vars {
            println!("{}={}", key, value);
        }
    }

    if args.shell {
        spawn_interactive_shell(&all_env_vars, &config_file)?;
    }

    Ok(())
}

fn spawn_interactive_shell(
    env_vars: &std::collections::BTreeMap<String, String>,
    config_file: &str,
) -> std::io::Result<()> {
    // Check if we're already in a NAIS environment shell
    if env::var("NAIS_ENV_ACTIVE").is_ok() {
        println!("Already in a NAIS environment shell. Not spawning a new one.");
        return Ok(());
    }

    let shell = if cfg!(target_os = "windows") {
        String::from("cmd")
    } else {
        env::var("SHELL").unwrap_or_else(|_| String::from("/bin/sh"))
    };

    // Create command for the shell
    let mut command = Command::new(shell);
    for (key, value) in env_vars {
        command.env(key, value);
    }

    // Make it interactive by setting stdio options
    command
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());

    let shell_path = env::var("SHELL").unwrap_or_else(|_| String::from("/bin/sh"));

    // Set the environment variable to indicate that the shell is active
    command.env("NAIS_ENV_ACTIVE", "true");
    command.env("NAIS_ENV_CONFIG", config_file);

    if shell_path.contains("bash") {
        command.env("PS1", "\\[\\e[32m\\][NAIS-ENV]\\[\\e[0m\\] \\w $ ");
    } else if shell_path.contains("zsh") {
        command.env("PROMPT", "%F{green}[NAIS-ENV]%f %~ $ ");
    } else {
        // Fallback for other shells
        command.env("PS1", "[NAIS-ENV] \\w $ ");
    }

    // Execute the command
    match command.spawn() {
        Ok(mut child) => {
            // Wait for the shell to exit
            match child.wait() {
                Ok(status) => println!("Shell exited with status: {}", status),
                Err(e) => eprintln!("Error waiting for shell: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to launch shell: {}", e),
    }

    Ok(())
}
