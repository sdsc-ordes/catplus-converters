use crate::convert::RdfFormat;

use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug, clap::ValueEnum, Clone)]
pub enum InputType {
    Synth,
    HCI,
    Agilent,
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

pub fn manage_input(input_path: &Path) -> Result<Option<InputType>> {
    let filename = input_path
        .file_name()
        .and_then(|f| f.to_str())
        .context("Invalid filename")?;

    if filename.ends_with(".ttl") || filename.ends_with(".jsonld") {
        println!("Skipping file '{}': already an RDF file.", filename);
        return Ok(None);
    }

    match detect_input_type(filename) {
        Some(input_type) => Ok(Some(input_type)),
        None => {
            println!("Skipping file '{}': no matching type.", filename);
            Ok(None)
        }
    }
}

pub fn manage_output(
    input_path: &Path,
    output_folder: &Path,
    serialized_graph: &str,
    format: &RdfFormat,
) -> Result<()> {
    let stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .context("Invalid file stem")?;
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

pub fn define_output_folder(input_path: &Path, user_specified: &Option<String>) -> Result<PathBuf> {
    let default_output_folder = if input_path.is_file() {
        input_path
            .parent()
            .map(Path::to_path_buf)
            .context("Failed to get parent folder of input file")?
    } else {
        input_path.to_path_buf()
    };

    let output_folder = match user_specified {
        Some(folder) => PathBuf::from(folder),
        None => default_output_folder,
    };

    Ok(output_folder)
}
