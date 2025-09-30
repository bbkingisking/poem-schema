use std::path::PathBuf;
use std::fs;
use anyhow::{Result, bail, anyhow};
use serde_yaml;
use serde_json;
use jsonschema::draft7;
use clap::Parser;
use rust_iso639::from_code_3;
use rust_iso15924::from_code;

const POEM_SCHEMA: &str = include_str!("../../../poem.schema.json");

#[derive(Parser)]
#[command(about = "A validator for .poem files")]
struct Args {
    // Path to the .poem file to validate
    poem_file: PathBuf,

    // Enforce strict ISO 639-3 + (optional) ISO-15924 language code validation
    #[arg(long)]
    strict_language: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let poem_file_string = fs::read_to_string(args.poem_file.clone())?;
    let poem_yaml: serde_yaml::Value = serde_yaml::from_str(&poem_file_string)?;
    validate_poem(poem_yaml, args.strict_language)?;
    println!("[OK] {} is valid", args.poem_file.file_name().unwrap().to_string_lossy());
    Ok(())
}

fn validate_poem(poem: serde_yaml::Value, strict_language: bool) -> Result<()> {
    validate_schema(&poem)?;

    if strict_language {
        validate_language_codes(&poem)?;
    };

    Ok(())
}

fn validate_schema(poem: &serde_yaml::Value) -> Result<()> {
    let parsed_schema: serde_json::Value = serde_json::from_str(POEM_SCHEMA)?;
    let json_poem = serde_json::to_value(poem)?;

    // Create a validator
    let validator = draft7::new(&parsed_schema)
        .map_err(|e| anyhow!("Invalid schema: {}", e))?;

    // Use iter_errors() to get detailed error messages
    let errors: Vec<String> = validator
        .iter_errors(&json_poem)
        .map(|error| format!("  ✖ {}", error))
        .collect();

    if errors.is_empty() {
        Ok(())
    } else {
        bail!("Schema validation failed:\n{:#?}", errors);
    }
}

fn validate_language_codes(poem_data: &serde_yaml::Value) -> Result<()> {
    if let serde_yaml::Value::Mapping(versions) = poem_data {
        for (version_key, version_data) in versions {
            match version_data.get("language") {
                Some(serde_yaml::Value::String(lang_code)) => {
                    if !is_valid_language_tag(lang_code) {
                        bail!("Invalid language tag '{}' in version '{:?}'", lang_code, version_key);
                    }
                },
                Some(_) => {
                    bail!("Language property must be a string in version '{:?}'", version_key);
                },
                None => {
                    bail!("Missing required language property in version '{:?}' (required with --force-iso)", version_key);
                }
            }
        }
    }
    Ok(())
}

fn is_valid_language_tag(tag: &str) -> bool {
    let parts: Vec<&str> = tag.split('-').collect();

    let language_part = parts[0];
    if from_code_3(language_part).is_none() {
        return false;
    }

    if parts.len() >= 2 {
        let script_part = parts[1];
        if from_code(script_part).is_none() {
            return false;
        }
    }
    true
}
