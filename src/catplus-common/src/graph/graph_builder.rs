use std::path::Path;

use crate::{
    graph::namespaces::{allores, cat, schema},
    rdf::rdf_serializers::{serialize_graph_to_jsonld, serialize_graph_to_turtle},
};
use anyhow::{Context, Result};
use sophia::inmem::graph::LightGraph;
use sophia_api::{prelude::*, term::SimpleTerm};

use super::insert_into::InsertIntoGraph;

/// An RDF Graph
pub struct GraphBuilder {
    pub graph: LightGraph,
}

/// Builds an RDF graph of Synthesis data for the cat+ ontology.
///
/// The rust structure `actions` in /parser/actions is mapped to the cat+ ontology
///
/// # public methods:
/// * insert:  starts the process of building the graph from the input structure
/// * serialize_to_turtle: serializes the graph to a turtle output
impl GraphBuilder {
    pub fn new() -> Self {
        Self { graph: LightGraph::new() }
    }

    /// Inserts a new object into the graph as a collection of triples.
    pub fn insert(&mut self, other: &dyn InsertIntoGraph) -> Result<()> {
        other.insert_into(&mut self.graph, other.get_uri())?;

        Ok(())
    }

    /// Adds a content URL to the graph.
    pub fn link_content(&mut self, content_url: &str) -> Result<()> {
        let campaign = &cat::Campaign.as_simple();
        let liquid_chromatography_document = &allores::AFR_0002524.as_simple();

        let triples = self
            .graph
            .triples_matching(Any, Any, [campaign, liquid_chromatography_document])
            .collect::<Result<Vec<_>, _>>()
            .map_err(anyhow::Error::from)?;

        // exit with warning if no triples are found.
        if triples.is_empty() {
            println!("Warning: No triples found for contentURL insertion.");
            return Ok(())
        } else if triples.len() > 1 {
            return Err(anyhow::anyhow!(
                "Multiple triples found for contentURL insertion"
            ));
        }
            
        let triple = triples.into_iter().next().unwrap();
        let [subject, _, _] = triple;

        match subject {
            SimpleTerm::Iri(subject_iri) => {
                self.graph.insert(
                    IriRef::new(subject_iri.as_str().to_owned()).unwrap(),
                    schema::contentURL.as_simple(),
                    content_url.as_simple(),
                )?;
            }
            SimpleTerm::BlankNode(subject_bnode) => {
                self.graph.insert(
                    subject_bnode.clone(),
                    schema::contentURL.as_simple(),
                    content_url.as_simple(),
                )?;
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Subject must be either an IRI or a BlankNode to add contentURL"
                ));
            }
        }

        Ok(())
    }

    /// Materializes blank nodes in the graph by replacing them with URIs.
    /// If a prefix is given, it will be used for all materialized blank nodes.
    /// Otherwise, the empty string is used as the prefix.
    pub fn materialize_blank_nodes(&mut self, prefix: Option<&str>) -> Result<()> {
        let mut materialized_graph = LightGraph::new();

        for triple in self.graph.triples_matching(Any, Any, Any) {
            let [subject, predicate, object] = triple?;

            // If the subject is a blank node, replace it with a URI
            let new_subject = match subject {
                SimpleTerm::BlankNode(s) => {
                    let new_iri = format!("{}{}", prefix.unwrap_or_default(), s.as_str());
                    IriRef::new(new_iri.to_owned())
                }
                SimpleTerm::Iri(s) => IriRef::new(s.as_str().to_owned()),
                _ => panic!("Unexpected subject type"),
            }?;

            // If the object is a blank node, replace it with a URI
            // In any other case, we just clone it.
            match object {
                SimpleTerm::BlankNode(o) => {
                    let new_o = format!("{}{}", prefix.unwrap_or_default(), o.as_str());
                    materialized_graph.insert(
                        new_subject,
                        predicate.clone(),
                        IriRef::new(new_o.as_str().to_owned()).unwrap(),
                    )?;
                }
                _ => {
                    materialized_graph.insert(new_subject, predicate.clone(), object.clone())?;
                }
            };
        }

        self.graph = materialized_graph;
        Ok(())
    }

    /// Get the turtle serialization of the RDF graph
    ///
    /// Assumes a new graph has been created and built.
    ///
    /// # Returns
    /// A `Result` containing the graph as Turtle serialization, or an error
    /// if the graph retrieval fails.
    pub fn serialize_to_turtle(&self) -> Result<String> {
        serialize_graph_to_turtle(&self.graph).context("Failed to serialize graph to Turtle")
    }

    /// Get the turtle serialization of the RDF graph
    ///
    /// Assumes a new graph has been created and built.
    ///
    /// # Returns
    ///  The `jsonld` serialization of the grap, or an error otherwise.
    /// if the graph retrieval fails.
    pub fn serialize_to_jsonld(&self) -> Result<String> {
        serialize_graph_to_jsonld(&self.graph).context("Failed to serialize graph to JSON-LD")
    }
}
