use anyhow::{Context, Result};
use catplus_common::models::{
    agilent::LiquidChromatographyAggregateDocumentWrapper, hci::CampaignWrapper, synth::SynthBatch,
};
use clap::Parser;
use converter::convert::{json_to_rdf, RdfFormat};
use serde::Deserialize;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

#[derive(Deserialize, Debug, clap::ValueEnum, Clone)]
enum InputType {
    Synth,
    HCI,
    Agilent,
}

#[derive(Parser, Debug)]
struct Args {
    /// Path to the input folder containing JSON files.
    input_folder: String,

    /// Path to the output folder.
    output_folder: String,

    /// Output RDF format: "Turtle" or "Jsonld".
    #[arg(value_enum)]
    format: RdfFormat,

    /// Materialize blank nodes
    #[arg(long, default_value_t = false)]
    materialize: bool,
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

fn process_file(
    input_path: &Path,
    output_path: &Path,
    input_type: InputType,
    format: &RdfFormat,
    materialize: bool,
) -> Result<()> {
    let mut input_content = String::new();
    File::open(input_path)
        .with_context(|| format!("Failed to open input file '{}'.", input_path.display()))?
        .read_to_string(&mut input_content)
        .with_context(|| format!("Failed to read input file '{}'.", input_path.display()))?;

    let serialized_graph = match input_type {
        InputType::Synth => json_to_rdf::<SynthBatch>(&input_content, format, materialize),
        InputType::HCI => json_to_rdf::<CampaignWrapper>(&input_content, format, materialize),
        InputType::Agilent => json_to_rdf::<LiquidChromatographyAggregateDocumentWrapper>(
            &input_content,
            format,
            materialize,
        ),
    }
    .with_context(|| {
        format!("Failed to convert '{}' to RDF format '{:?}'", input_path.display(), format)
    })?;

    let mut output_file = File::create(output_path)
        .with_context(|| format!("Failed to create output file '{}'.", output_path.display()))?;
    output_file
        .write_all(serialized_graph.as_bytes())
        .with_context(|| format!("Failed to write to output file '{}'.", output_path.display()))?;

    println!("Processed '{}' -> '{}'", input_path.display(), output_path.display());

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let input_folder = Path::new(&args.input_folder);
    let output_folder = Path::new(&args.output_folder);

    if !input_folder.is_dir() {
        anyhow::bail!("Input path '{}' is not a folder.", input_folder.display());
    }

    fs::create_dir_all(output_folder).with_context(|| {
        format!("Failed to create output folder '{}'.", output_folder.display())
    })?;

    for entry in fs::read_dir(input_folder)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let filename = path.file_name().and_then(|f| f.to_str()).unwrap_or("");

            if filename.ends_with(".ttl") || filename.ends_with(".jsonld") {
                println!("Skipping file '{}': already an RDF file.", filename);
                continue;
            }

            if let Some(input_type) = detect_input_type(filename) {
                let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                let output_filename = format!("{}.{}", stem, args.format.extension());
                let output_path = output_folder.join(output_filename);

                process_file(&path, &output_path, input_type, &args.format, args.materialize)?;
            } else {
                println!("Skipping file '{}': no matching type.", filename);
            }
        }
    }

    println!("All files processed.");
    Ok(())
}
trait RdfFormatExt {
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
