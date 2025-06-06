use regex::Regex;
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

/// Gets a value from a YAML structure using a dotted path notation
///
/// # Arguments
/// * `yaml_value` - The YAML value to extract from
/// * `path` - The dot-separated path to the desired value (e.g., "app.name")
///
/// # Returns
/// A string representation of the value or empty string if not found
fn get_value_at_path(yaml_value: &Value, path: &str) -> String {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = yaml_value;

    // Navigate through the nested structure
    for &part in &parts {
        match current {
            Value::Mapping(map) => {
                // Try with string key
                let key = Value::String(part.to_string());
                if let Some(value) = map.get(&key) {
                    current = value;
                    continue;
                }

                // Key not found
                return String::new();
            }
            _ => return String::new(), // Not a mapping, can't navigate further
        }
    }

    // Convert the final value to a string
    match current {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        Value::Sequence(seq) => {
            let items: Vec<String> = seq
                .iter()
                .map(|v| match v {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    _ => format!("{:?}", v),
                })
                .collect();
            items.join(",")
        }
        _ => format!("{:?}", current),
    }
}

/// Substitutes variables in a string using the format {{ variable.path }}
///
/// # Arguments
/// * `content` - The string content to perform substitution on
/// * `variables` - The YAML value containing all variables
///
/// # Returns
/// A new string with all variables substituted
///
/// # Example
/// ```
/// let content = "apiVersion: \"nais.io/v1alpha1\"\nkind: \"Application\"\nmetadata:\n  name: \"{{ app.name }}\"";
/// let variables = parse_variables_file("vars.yaml")?;
/// let result = substitute_variables(content, &variables);
/// ```
pub fn substitute_variables(content: &str, variables: &Value) -> String {
    // Regex to match {{ var.path }} patterns, both quoted and unquoted
    let re = Regex::new(r#"(?:"?\{\{\s*([^{}]+?)\s*\}\}"?|\{\{\s*([^{}]+?)\s*\}\})"#).unwrap();

    let result = re.replace_all(content, |caps: &regex::Captures| {
        // Check which capture group matched (with or without quotes)
        let path = if let Some(m) = caps.get(1) {
            m.as_str().trim()
        } else if let Some(m) = caps.get(2) {
            m.as_str().trim()
        } else {
            return String::new();
        };

        get_value_at_path(variables, path)
    });

    result.to_string()
}
