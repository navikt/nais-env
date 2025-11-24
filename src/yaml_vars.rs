use handlebars::Handlebars;
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

/// Substitutes variables in a string using Handlebars template syntax
///
/// This function supports:
/// - Simple variable substitution: {{ variable.path }}
/// - Loops: {{#each array}} ... {{/each}}
/// - Conditionals: {{#if condition}} ... {{/if}}
/// - And other Handlebars features
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
/// let content = "name: {{ app.name }}";
/// let variables = parse_variables_file("vars.yaml")?;
/// let result = substitute_variables(content, &variables);
/// ```
pub fn substitute_variables(content: &str, variables: &Value) -> String {
    let mut handlebars = Handlebars::new();

    // Disable HTML escaping since we're working with YAML, not HTML
    handlebars.register_escape_fn(handlebars::no_escape);

    // Register the template
    match handlebars.register_template_string("template", content) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error registering template: {}", e);
            return content.to_string();
        }
    }

    // Convert YAML Value to serde_json::Value for Handlebars compatibility
    let json_value = yaml_to_json(variables);

    // Render the template
    match handlebars.render("template", &json_value) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error rendering template: {}", e);
            content.to_string()
        }
    }
}

/// Converts a serde_yaml::Value to serde_json::Value
///
/// This is necessary because Handlebars works with serde_json::Value
/// but we parse our variables as YAML.
fn yaml_to_json(yaml: &Value) -> serde_json::Value {
    match yaml {
        Value::Null => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                serde_json::Value::Number(serde_json::Number::from(i))
            } else if let Some(u) = n.as_u64() {
                serde_json::Value::Number(serde_json::Number::from(u))
            } else if let Some(f) = n.as_f64() {
                serde_json::Number::from_f64(f)
                    .map(serde_json::Value::Number)
                    .unwrap_or(serde_json::Value::Null)
            } else {
                serde_json::Value::Null
            }
        }
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::Sequence(seq) => {
            let arr: Vec<serde_json::Value> = seq.iter().map(yaml_to_json).collect();
            serde_json::Value::Array(arr)
        }
        Value::Mapping(map) => {
            let mut obj = serde_json::Map::new();
            for (k, v) in map {
                if let Value::String(key) = k {
                    obj.insert(key.clone(), yaml_to_json(v));
                }
            }
            serde_json::Value::Object(obj)
        }
        Value::Tagged(tagged) => yaml_to_json(&tagged.value),
    }
}
