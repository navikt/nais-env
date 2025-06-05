use std::{env, io, process::Command};

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{
    generate,
    shells::{Bash, Fish, PowerShell, Zsh},
};
mod env_file;
mod git;
mod kubernetes_client;
mod nais;
mod yaml_vars;

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
    config: Option<String>,

    /// YAML file containing variables
    #[arg(short, long)]
    variables: Option<String>,

    /// Print secrets
    #[arg(short, long)]
    print: bool,

    /// Spawn shell with secrets as environment variables or run specified command
    #[arg(short, long, default_missing_value = "SHELL" , num_args = 0..=1)]
    shell: Option<String>,

    /// Clear all files added by nais-env (must be in git repository)
    #[arg(long)]
    clear_files: bool,

    /// Kubernetes context to use (only 'nais-dev' or 'dev-fss' are supported), defaults to nais-dev
    #[arg(long, default_value = "nais-dev")]
    context: String,

    /// Subcommands
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Subcommands
#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate shell completion scripts
    Completion {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: CompletionShell,
    },
}

/// Supported shells for completion
#[derive(clap::ValueEnum, Clone, Debug)]
enum CompletionShell {
    /// Bash shell
    Bash,
    /// Zsh shell
    Zsh,
    /// Fish shell
    Fish,
    /// PowerShell
    PowerShell,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // Handle completions subcommand
    if let Some(Commands::Completion { shell }) = &args.command {
        return generate_completion(shell);
    }

    if args.clear_files {
        match env_file::clear_env_files() {
            Ok(_) => std::process::exit(0),
            Err(e) => {
                eprintln!("Error clearing env files: {}", e);
                std::process::exit(1);
            }
        }
    }

    const ALLOWED_CONTEXTS: [&str; 2] = ["nais-dev", "dev-fss"];

    // Check if context is allowed, if not exit with error
    if !ALLOWED_CONTEXTS.contains(&args.context.as_str()) {
        eprintln!(
            "Error: Invalid context '{}'. Must be one of: {}",
            args.context,
            ALLOWED_CONTEXTS.join(", ")
        );
        std::process::exit(1);
    }

    let overrides = if let Some(override_files) = args.overrides {
        env_file::parse_multiple_env_files(override_files)?
    } else {
        std::collections::BTreeMap::new()
    };

    let config_file = match args.config {
        Some(path) => path,
        None => {
            eprintln!("Error: Missing --config[-c] argument. Please provide a path to nais.yaml");
            std::process::exit(1);
        }
    };

    // Get variable file if provided
    let nais_config = if let Some(var_file) = args.variables {
        match yaml_vars::parse_variables_file(&var_file) {
            Ok(variables) => {
                nais::NaisConfigLoader::new_with_variables(config_file.clone(), variables)
            }
            Err(e) => {
                eprintln!("Error parsing variables file '{}': {}", var_file, e);
                std::process::exit(1);
            }
        }
    } else {
        nais::NaisConfigLoader::new(config_file.clone())
    }
    .expect("Could not load config file");

    let kubernetes_client = kubernetes_client::KubernetesClient::new(
        nais_config.get_namespace(),
        nais_config.get_deployment(),
        args.context,
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

    if let Some(file) = args.file {
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

    if let Some(shell_value) = args.shell.as_deref() {
        let shell_command = if shell_value == "SHELL" {
            None
        } else {
            Some(shell_value)
        };
        spawn_interactive_shell(&all_env_vars, &config_file, shell_command)?;
    }

    Ok(())
}

fn spawn_interactive_shell(
    env_vars: &std::collections::BTreeMap<String, String>,
    config_file: &str,
    shell_command: Option<&str>,
) -> Result<(), std::io::Error> {
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
    let mut command = if let Some(cmd) = shell_command {
        let cmd_args = if cfg!(target_os = "windows") {
            vec!["/C", cmd]
        } else {
            vec!["-c", cmd]
        };
        let mut c = Command::new(&shell);
        c.args(&cmd_args);
        c
    } else {
        Command::new(shell)
    };

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
        command.env(
            "PS1",
            "\\[\\e[32m\\][NAIS-ENV:$NAIS_ENV_CONFIG]\\[\\e[0m\\] \\w $ ",
        );
    } else {
        // Fallback for other shells
        command.env("PS1", "[NAIS-ENV:$NAIS_ENV_CONFIG] \\w $ ");
    }

    // Execute the command
    match command.spawn() {
        Ok(mut child) => match child.wait() {
            Ok(status) => {
                if shell_command.is_some() {
                    println!("Command exited with status: {}", status);
                } else {
                    println!("Shell exited with status: {}", status);
                }
                Ok(())
            }
            Err(e) => {
                eprintln!("Error waiting for process: {}", e);
                Err(e)
            }
        },
        Err(e) => {
            if shell_command.is_some() {
                eprintln!("Failed to execute command: {}", e);
            } else {
                eprintln!("Failed to launch shell: {}", e);
            }
            Err(e)
        }
    }
}

/// Generate shell completion scripts
fn generate_completion(shell: &CompletionShell) -> io::Result<()> {
    let mut cmd = Args::command();
    let bin_name = cmd.get_name().to_string();

    match shell {
        CompletionShell::Bash => {
            generate(Bash, &mut cmd, bin_name, &mut io::stdout());
        }
        CompletionShell::Zsh => {
            generate(Zsh, &mut cmd, bin_name, &mut io::stdout());
        }
        CompletionShell::Fish => {
            generate(Fish, &mut cmd, bin_name, &mut io::stdout());
        }
        CompletionShell::PowerShell => {
            generate(PowerShell, &mut cmd, bin_name, &mut io::stdout());
        }
    }

    Ok(())
}
