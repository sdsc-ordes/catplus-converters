use std::path::Path;

use crate::rdf::rdf_serializers::{serialize_graph_to_jsonld, serialize_graph_to_turtle};
use anyhow::{Context, Result};
use sophia::inmem::graph::LightGraph;
use sophia_api::{prelude::*, term::SimpleTerm};
use uuid::fmt::Simple;
use crate::graph::namespaces::schema;

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
    pub fn add_content(&mut self, content_url: &Path) -> Result<()> {
        let triple = {
            let triples = self
                .graph
                .triples_matching(Any, Any, ["Campaign", "AFR_0002524"])
                .collect::<Result<Vec<_>, _>>()
                .map_err(anyhow::Error::from)?;
    
            triples
                .into_iter()
                .next()
                .ok_or_else(|| anyhow::anyhow!("No triples found"))?
        };
    
        let [subject, _, _] = triple;
    
        if let SimpleTerm::Iri(subject_iri) = subject {
            self.graph.insert(
                IriRef::new(subject_iri.as_str().to_owned()).unwrap(),
                schema::contentURL.as_simple(),
                content_url
                    .to_str()
                    .ok_or_else(|| anyhow::anyhow!("Invalid UTF-8 in path"))?
                    .as_simple(),
            )?;
        } else {
            return Err(anyhow::anyhow!("Unable to add contentURL to graph"));
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
