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
