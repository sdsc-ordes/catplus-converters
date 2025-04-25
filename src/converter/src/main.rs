use catplus_common::models::{
    agilent::LiquidChromatographyAggregateDocumentWrapper, bravo::BravoActionWrapper,
    hci::CampaignWrapper, synth::SynthBatch,
};
use converter::{
    convert::{json_to_rdf, RdfFormat},
    manage_io::{
        define_output_folder, determine_input_action, save_output, InputAction, InputType,
    },
};

use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

#[derive(Parser, Debug)]
struct Args {
    /// Path to the input file or folder containing files.
    input_path: String,
    /// batch/2025/10/01/24/ ....

    /// Path to the output folder. Defaults to input folder if not specified.
    #[arg(long)]
    output_folder: Option<String>,

    /// Output RDF format: "Turtle" or "Jsonld".
    #[arg(value_enum)]
    format: RdfFormat,

    /// Materialize blank nodes
    #[arg(long, default_value_t = false)]
    materialize: bool,
}

fn process_file(
    input_path: &Path,
    output_folder: &Path,
    format: &RdfFormat,
    materialize: bool,
) -> Result<()> {
    let input_type = match determine_input_action(input_path)? {
        InputAction::Skip(reason) => {
            println!("Skipping file '{}': {}", input_path.display(), reason);
            return Ok(());
        }
        InputAction::Process(input_type) => input_type,
    };

    let serialized_graph = match input_type {
        InputType::HCI => json_to_rdf::<CampaignWrapper>(&input_path, format, materialize),
        InputType::Synth => json_to_rdf::<SynthBatch>(&input_path, format, materialize),
        InputType::Agilent => json_to_rdf::<LiquidChromatographyAggregateDocumentWrapper>(
            &input_path,
            format,
            materialize,
        ),
        InputType::Bravo => json_to_rdf::<BravoActionWrapper>(&input_path, format, materialize),
    }
    .with_context(|| {
        format!("Failed to convert '{}' to RDF format '{:?}'", input_path.display(), format)
    })?;

    save_output(input_path, output_folder, &serialized_graph, format)?;

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let input_path = Path::new(&args.input_path);
    if !input_path.exists() {
        anyhow::bail!("Input path '{}' does not exist.", input_path.display());
    }

    let output_folder = define_output_folder(input_path, &args.output_folder)?;

    fs::create_dir_all(&output_folder).with_context(|| {
        format!("Failed to create output folder '{}'.", output_folder.display())
    })?;

    if input_path.is_file() {
        process_file(input_path, &output_folder, &args.format, args.materialize)?;
    } else if input_path.is_dir() {
        for entry in fs::read_dir(input_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                process_file(&path, &output_folder, &args.format, args.materialize)?;
            }
        }
    } else {
        anyhow::bail!("Input path '{}' is neither a file nor a directory.", input_path.display());
    }

    println!("All files processed.");
    Ok(())
}
