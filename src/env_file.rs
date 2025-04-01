use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Reads and parses an environment file into a BTreeMap
///
/// # Arguments
///
/// * `file_path` - Path to the environment file
///
/// # Returns
///
/// * `Result<BTreeMap<String, String>, io::Error>` - BTreeMap of key-value pairs or an error
pub fn parse_env_file<P: AsRef<Path>>(file_path: P) -> Result<BTreeMap<String, String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut env_vars = BTreeMap::new();

    for line in reader.lines() {
        let line = line?;

        // Skip empty lines and comments
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Parse key=value pairs
        if let Some(idx) = trimmed.find('=') {
            let key = trimmed[..idx].trim();
            let value = trimmed[(idx + 1)..].trim();

            // Remove quotes if they exist
            let value = value.trim_matches(|c| c == '\'' || c == '"');

            if !key.is_empty() {
                env_vars.insert(key.to_string(), value.to_string());
            }
        }
    }

    Ok(env_vars)
}

/// Reads and parses multiple environment files into a single BTreeMap
///
/// # Arguments
///
/// * `file_paths` - Vector of paths to environment files
///
/// # Returns
///
/// * `Result<BTreeMap<String, String>, io::Error>` - Combined BTreeMap of key-value pairs or an error
pub fn parse_multiple_env_files<P: AsRef<Path>>(
    file_paths: Vec<P>,
) -> Result<BTreeMap<String, String>, io::Error> {
    let mut combined_env_vars = BTreeMap::new();

    for path in file_paths {
        let env_vars = parse_env_file(path)?;
        // Extend the combined map with values from each file
        // Later files will override earlier files if keys conflict
        combined_env_vars.extend(env_vars);
    }

    Ok(combined_env_vars)
}

pub fn save_env_vars_to_file(
    filename: &str,
    env_vars: &std::collections::BTreeMap<String, String>,
) -> std::io::Result<()> {
    // Convert relative path to absolute path
    let path = std::path::Path::new(filename);

    // Create file for writing
    let mut file = std::fs::File::create(path)?;

    // Write each environment variable as KEY=VALUE
    for (key, value) in env_vars {
        use std::io::Write;
        writeln!(file, "{}={}", key, value)?;
    }

    // If in a git repository make sure git ignores file by adding it to .git/info/exclude
    if let Ok(output) = std::process::Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
    {
        if output.status.success() {
            let git_dir = std::process::Command::new("git")
                .args(["rev-parse", "--git-dir"])
                .output()
                .ok()
                .and_then(|output| String::from_utf8(output.stdout).ok())
                .map(|s| s.trim().to_string());

            if let Some(git_dir) = git_dir {
                let exclude_path = std::path::Path::new(&git_dir).join("info/exclude");
                if exclude_path.exists() {
                    let file_name = path.file_name().unwrap_or_default().to_string_lossy();

                    // Check if the file is already in the exclude file
                    let exclude_content =
                        std::fs::read_to_string(&exclude_path).unwrap_or_default();
                    let lines: Vec<&str> = exclude_content.lines().collect();

                    // Only add if it doesn't already exist
                    if !lines.contains(&file_name.as_ref()) {
                        if !lines.contains(&"# Added by nais-env") {
                            // Add comment and filename at the end if comment doesn't exist
                            if let Ok(mut exclude_file) =
                                std::fs::OpenOptions::new().append(true).open(&exclude_path)
                            {
                                use std::io::Write;
                                let _ = writeln!(exclude_file, "# Added by nais-env");
                                let _ = writeln!(exclude_file, "{}", file_name);
                            }
                        } else {
                            // If comment exists, insert filename right after it
                            let mut new_content = String::new();
                            let mut inserted = false;

                            for line in lines {
                                new_content.push_str(line);
                                new_content.push('\n');

                                if line == "# Added by nais-env" && !inserted {
                                    new_content.push_str(&format!("{}\n", file_name));
                                    inserted = true;
                                }
                            }

                            if let Ok(_) = std::fs::write(&exclude_path, new_content) {
                                println!(
                                    "Added {} to local git exclude file (.git/info/exclude)",
                                    file_name
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
/// Deletes all files listed under the "# Added by nais-env" comment in .git/info/exclude
///
/// # Returns
///
/// * `io::Result<()>` - Success or error
pub fn clear_env_files() -> io::Result<()> {
    // Check if we're in a git repository
    if let Ok(output) = std::process::Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
    {
        if output.status.success() {
            // Get git directory
            let git_dir = std::process::Command::new("git")
                .args(["rev-parse", "--git-dir"])
                .output()
                .ok()
                .and_then(|output| String::from_utf8(output.stdout).ok())
                .map(|s| s.trim().to_string());

            if let Some(git_dir) = git_dir {
                let exclude_path = std::path::Path::new(&git_dir).join("info/exclude");
                if exclude_path.exists() {
                    // Read the exclude file
                    let exclude_content = std::fs::read_to_string(&exclude_path)?;
                    let lines: Vec<&str> = exclude_content.lines().collect();

                    // Find the "# Added by nais-env" comment
                    let mut files_to_delete: Vec<String> = Vec::new();
                    let mut in_nais_env_section = false;
                    let mut updated_content = String::new();

                    // Parse the exclude file
                    for line in lines {
                        if line == "# Added by nais-env" {
                            in_nais_env_section = true;
                            continue; // Skip this line in the updated content
                        } else if in_nais_env_section {
                            if line.is_empty() {
                                in_nais_env_section = false;
                                updated_content.push_str("\n");
                            } else {
                                files_to_delete.push(line.to_string());
                                continue; // Skip this line in the updated content
                            }
                        } else {
                            updated_content.push_str(line);
                            updated_content.push('\n');
                        }
                    }

                    // Delete the identified files
                    for file in &files_to_delete {
                        let file_path = std::path::Path::new(file);
                        if file_path.exists() {
                            std::fs::remove_file(file_path)?;
                            println!("Deleted env file: {}", file);
                        }
                    }

                    // Update the exclude file to remove the entries
                    if !files_to_delete.is_empty() {
                        std::fs::write(exclude_path, updated_content)?;
                        println!(
                            "Removed {} entries from git exclude file",
                            files_to_delete.len()
                        );
                    }
                }
            }
        }
    }

    Ok(())
}
