mod nais;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let nais_config_path = if args.len() > 1 {
        &args[1]
    } else {
        panic!("Missing config file")
    };
    println!("Using NAIS config: {}", nais_config_path);

    let nais_config = nais::load_config(&nais_config_path);

    let nais_config = match nais_config {
        Ok(config) => {
            if config.kind != "Application" {
                panic!("Expected kind 'Application', got '{}'", config.kind);
            }
            config
        }
        Err(e) => panic!("Failed to load NAIS config: {}", e),
    };

    println!("YAML content: {:#?}", nais_config);

    println!("Parsed YAML successfully");
}
