use anyhow::{Context, Result};
use catplus_common::graph::{graph_builder::GraphBuilder, insert_into::InsertIntoGraph};
use serde::{de::DeserializeOwned, Deserialize};
use std::{
    convert::TryFrom,
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

// Derive Deserialize and ValueEnum
#[derive(Deserialize, Debug, clap::ValueEnum, Clone)]
pub enum RdfFormat {
    Turtle,
    Jsonld,
}

/// Configuration struct for the converter
#[derive(Clone, Debug)]
pub struct ConverterConfig {
    pub input_path: PathBuf,
    pub format: RdfFormat,
    pub prefix: Option<String>,
    pub materialize: bool,
}


fn read_to_string(path: &Path) -> Result<String> {
    let mut content = String::new();
    File::open(path)
        .with_context(|| format!("Failed to open file '{}'.", path.display()))?
        .read_to_string(&mut content)
        .with_context(|| format!("Failed to read file '{}'.", path.display()))?;
    Ok(content)
}

/// Builds the content file URI using an absolute path, or prefix and relative path.
fn build_file_uri(prefix: Option<String>, path: &Path) -> Result<String> {

    if let Some(ref p) = prefix {
        if p.is_empty() {
            return Err(anyhow::anyhow!(
                "Cannot use empty prefix."
            ));
        }
    }

    match (prefix, path.is_absolute()) {
        // Prefix, Relative path
        (Some(p), false) => {
            return Ok(format!("{}{}", p, path.to_string_lossy()))
        },
        // No prefix, relative path
        (_, false) => {
            return Err(anyhow::anyhow!(
            "Cannot build URI for relative path without a prefix."
            ))
        },
        // Absolute path -> ignore prefix
        (p, true) => {
            if p.is_some() {
                println!("Prefix is ignored with absolute paths")
            }
            return Ok(format!("file://{}", path.to_string_lossy()))
        },
    }
}

/// Parses JSON and serializes the RDF graph to the specified format.
///
/// This function can handle any struct that implements `serde::DeserializeOwned` and your `InsertIntoGraph` trait.
///
/// # Arguments
/// - `input_content`: The JSON input as a string.
/// - `format`: The desired serialization format.
///
/// # Returns
/// A `Result` containing the serialized graph as a string or an error.
pub fn json_to_rdf<T>(config: &ConverterConfig) -> Result<String>
where
    T: DeserializeOwned + InsertIntoGraph, // Trait bounds
{

    let mut input_content = read_to_string(Path::new(&config.input_path))?;
    let instances: T = parse_json(&input_content).context("Failed to parse JSON input")?;
    let mut builder = GraphBuilder::new();
    builder.insert(&instances)?;

    let uri = build_file_uri(config.prefix.clone(), Path::new(&config.input_path))
        .context("Failed to build file URI")?;
    builder.link_content(&uri)
        .context("Failed to add content URL to the graph")?;

    if config.materialize {
        builder
            .materialize_blank_nodes(Some("http://example.org/cat/resource/"))
            .context("Failed to materialize blank nodes")?;
    }

    let serialized_graph = match &config.format {
        RdfFormat::Jsonld => {
            builder.serialize_to_jsonld().context("Failed to serialize to JSON-LD")?
        }
        RdfFormat::Turtle => {
            builder.serialize_to_turtle().context("Failed to serialize to Turtle")?
        }
    };

    Ok(serialized_graph)
}

/// Parses a JSON string into a struct of type T.
fn parse_json<T>(json_data: &str) -> Result<T>
where
    T: DeserializeOwned, // Trait bound
{
    serde_json::from_str(json_data).map_err(|e| anyhow::Error::new(e))
}
