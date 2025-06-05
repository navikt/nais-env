use serde_yaml::Value;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Parses a YAML file containing variables and returns the parsed YAML value.
///
/// The YAML file should contain variables that can be used for substitution
/// in configuration files.
///
/// # Arguments
/// * `file_path` - Path to the YAML file containing variables
///
/// # Returns
/// A result containing the parsed YAML structure or an error if parsing fails
///
/// # Errors
/// This function will return an error if:
/// * The file cannot be read
/// * The YAML cannot be parsed
///
/// # Example
/// ```
/// let variables = parse_variables_file("vars.yaml")?;
/// ```
pub fn parse_variables_file<P: AsRef<Path>>(
    file_path: P,
) -> Result<Value, Box<dyn std::error::Error>> {
    // Read the file contents
    let mut file = File::open(file_path.as_ref())?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Parse YAML content as a Value
    let variables: Value = serde_yaml::from_str(&content)?;

    Ok(variables)
}
