use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use crate::git;

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

/// Saves environment variables to a file and adds it to Git's exclude list if in a Git repository
///
/// # Arguments
///
/// * `filename` - Path to the file where environment variables will be saved
/// * `env_vars` - BTreeMap containing environment variables as key-value pairs
///
/// # Returns
///
/// * `io::Result<()>` - Success or error
///
/// # Details
///
/// This function:
/// 1. Creates a file and writes environment variables in the format `KEY=VALUE`
/// 2. If in a Git repository, adds the file to `.git/info/exclude` to prevent accidental commits
/// 3. Groups added files under a "# Added by nais-env" comment in the exclude file
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
    if git::is_in_git_repo() {
        if let Some(exclude_path) = git::get_git_exclude_path() {
            if exclude_path.exists() {
                let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                let exclude_content = std::fs::read_to_string(&exclude_path).unwrap_or_default();
                let lines: Vec<&str> = exclude_content.lines().collect();

                if !lines.contains(&file_name.as_ref()) {
                    if let Err(e) = git::add_to_git_exclude(path) {
                        eprintln!("Warning: Failed to add file to git exclude: {}", e);
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
    if !git::is_in_git_repo() {
        return Ok(());
    }

    // Get git exclude path
    let exclude_path = match git::get_git_exclude_path() {
        Some(path) if path.exists() => path,
        _ => return Ok(()),
    };

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

    Ok(())
}
