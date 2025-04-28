use crate::convert::RdfFormat;

use anyhow::{Context, Result};
use serde::Deserialize;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Debug, clap::ValueEnum, Clone)]
pub enum InputType {
    Synth,
    HCI,
    Agilent,
    Bravo,
}

pub trait RdfFormatExt {
    fn extension(&self) -> &'static str;
}

impl RdfFormatExt for RdfFormat {
    fn extension(&self) -> &'static str {
        match self {
            RdfFormat::Turtle => "ttl",
            RdfFormat::Jsonld => "jsonld",
        }
    }
}

fn detect_input_type(filename: &str) -> Option<InputType> {
    let lowercase = filename.to_lowercase();
    if lowercase.contains("synth") {
        Some(InputType::Synth)
    } else if lowercase.contains("hci") {
        Some(InputType::HCI)
    } else if lowercase.contains("agilent") {
        Some(InputType::Agilent)
    } else {
        None
    }
}

pub enum InputAction {
    Skip(String), // reason
    Process(InputType),
}
/// Decide what action to take on an input file.
pub fn determine_input_action(input_path: &Path) -> Result<InputAction> {
    let filename = input_path
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Input path has no filename"))?
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Filename is not valid UTF-8"))?;
    if filename.ends_with(".ttl") || filename.ends_with(".jsonld") {
        return Ok(InputAction::Skip("Unknown extension.".to_string()));
    }

    let action = match detect_input_type(filename) {
        Some(input_type) => InputAction::Process(input_type),
        None => InputAction::Skip("No matching type.".to_string()),
    };
    return Ok(action);
}

pub fn save_output(
    input_path: &Path,
    output_folder: &Path,
    serialized_graph: &str,
    format: &RdfFormat,
) -> Result<()> {
    let stem = input_path.file_stem().and_then(|s| s.to_str()).context("Invalid file stem")?;
    let output_filename = format!("{}.{}", stem, format.extension());
    let output_path = output_folder.join(output_filename);

    let mut output_file = File::create(&output_path)
        .with_context(|| format!("Failed to create output file '{}'.", output_path.display()))?;
    output_file
        .write_all(serialized_graph.as_bytes())
        .with_context(|| format!("Failed to write to output file '{}'.", output_path.display()))?;

    println!("Processed '{}' -> '{}'", input_path.display(), output_path.display());

    Ok(())
}

/// Defines a fallback output folder based on input file in case  it is missing.
pub fn define_output_folder(input_path: &PathBuf, provided_output_folder: &Option<PathBuf>) -> Result<PathBuf> {
    let default_output_folder = if input_path.is_file() {
        input_path
            .parent()
            .context("Failed to get parent folder of input file")?
    } else {
        input_path
    };

    let output_folder = match provided_output_folder {
        Some(folder) => PathBuf::from(folder),
        _ => default_output_folder.to_path_buf(),
    };

    Ok(output_folder)
}
