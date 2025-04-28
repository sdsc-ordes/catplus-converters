use catplus_common::models::{
    agilent::LiquidChromatographyAggregateDocumentWrapper, bravo::BravoActionWrapper,
    hci::CampaignWrapper, synth::SynthBatch,
};
use converter::{
    convert::{json_to_rdf, ConverterConfig, RdfFormat},
    manage_io::{
        define_output_folder, determine_input_action, save_output, InputAction, InputType,
    },
};
use sophia::iri::wrap;
use std::convert::Into;

use anyhow::{Context, Result};
use clap::{builder, Parser};
use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
struct Args {
    /// Path to the input file or folder containing files.
    input_path: PathBuf,
    /// batch/2025/10/01/24/ ....

    /// Path to the output folder. Defaults to input folder if not specified.
    #[arg(long)]
    output_folder: Option<PathBuf>,

    /// Output RDF format: "Turtle" or "Jsonld".
    #[arg(value_enum)]
    format: RdfFormat,

    /// Prefix to use for relative content URIs.
    #[arg(long)]
    prefix: Option<String>,

    /// Materialize blank nodes
    #[arg(long, default_value_t = false)]
    materialize: bool,
}

impl Into<ConverterConfig> for Args {
    fn into(self) -> ConverterConfig {
        ConverterConfig {
            input_path: self.input_path,
            format: self.format,
            prefix: self.prefix,
            materialize: self.materialize,
        }
    }
}

fn process_file(config: ConverterConfig, output_folder: &Path) -> Result<()> {
    let input_path = config.input_path.clone();
    let format = config.format.clone();
    let input_type = match determine_input_action(input_path.as_path())? {
        InputAction::Skip(reason) => {
            println!("Skipping file '{}': {}", input_path.display(), reason);
            return Ok(());
        }
        InputAction::Process(input_type) => input_type,
    };

    let serialized_graph = match input_type {
        InputType::HCI => json_to_rdf::<CampaignWrapper>(&config),
        InputType::Synth => json_to_rdf::<SynthBatch>(&config),
        InputType::Agilent => json_to_rdf::<LiquidChromatographyAggregateDocumentWrapper>(&config),
        InputType::Bravo => json_to_rdf::<BravoActionWrapper>(&config),
    }
    .with_context(|| {
        format!("Failed to convert '{}' to RDF format '{:?}'", input_path.display(), format)
    })?;

    save_output(&config.input_path, &output_folder, &serialized_graph, &config.format)?;

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let input_path = args.input_path.clone();
    let output_folder = args.output_folder.clone();
    let config: ConverterConfig = args.into();
    if !input_path.exists() {
        anyhow::bail!("Input path '{}' does not exist.", input_path.display());
    }

    let output_folder = define_output_folder(&input_path, &output_folder)?;

    fs::create_dir_all(&output_folder).with_context(|| {
        format!("Failed to create output folder '{}'.", output_folder.display())
    })?;

    if input_path.is_file() {
        process_file(config, &output_folder)?;
    } else if input_path.is_dir() {
        let mut entry_config = config.clone();
        for entry in fs::read_dir(input_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                entry_config.input_path = path;
                process_file(entry_config.clone(), &output_folder)?;
            }
        }
    } else {
        anyhow::bail!("Input path '{}' is neither a file nor a directory.", input_path.display());
    }

    println!("All files processed.");
    Ok(())
}
