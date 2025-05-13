use clap::Parser;
use serde_json;
use serde_yaml;
use std::fs;
use std::path::Path;

/// CLI tool to convert between JSON and YAML formats.
#[derive(Parser)]
#[command(name = "jsonyaml", author, version, about, long_about = None)]
struct Cli {
    /// Path to the input file (JSON or YAML)
    file_path: String,
}

fn main() {
    let args = Cli::parse();
    let file_path = args.file_path;

    if let Err(e) = convert_file(&file_path) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn convert_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(file_path)?;
    let path = Path::new(file_path);

    if let Some(extension) = path.extension() {
        match extension.to_str().unwrap_or("") {
            "json" => {
                let yaml: serde_yaml::Value = serde_json::from_str(&input)?;
                let output = serde_yaml::to_string(&yaml)?;
                let output_path = path.with_extension("yaml");
                fs::write(output_path, output)?;
                println!("Converted JSON to YAML successfully.");
            }
            "yaml" | "yml" => {
                let json: serde_json::Value = serde_yaml::from_str(&input)?;
                let output = serde_json::to_string_pretty(&json)?;
                let output_path = path.with_extension("json");
                fs::write(output_path, output)?;
                println!("Converted YAML to JSON successfully.");
            }
            _ => {
                return Err("Unsupported file extension. Please provide a .json or .yaml file.".into());
            }
        }
    } else {
        return Err("File has no extension. Please provide a valid .json or .yaml file.".into());
    }

    Ok(())
}
