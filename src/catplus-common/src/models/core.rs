#[rustfmt::skip]
// The structure follows the input data as descibed in the
// https://github.com/sdsc-ordes/catplus-ontology see here for the expected Synth input data:
// https://github.com/sdsc-ordes/catplus-ontology/tree/96091fd2e75e03de8a4c4d66ad502b2db27998bd/json-file/1-Synth
use crate::{
    graph::{
        insert_into::{InsertIntoGraph, Link},
        namespaces::{allores, cat, cat_resource, obo, purl, qudt, schema},
    },
    models::enums::Unit,
    graph::utils::hash_identifier,
};
use anyhow;
use serde::{Deserialize, Serialize};
use sophia::{
    api::{ns::rdf, prelude::*},
    inmem::graph::LightGraph,
};
use sophia_api::term::{SimpleTerm, Term};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plate {
    #[serde(rename = "containerID")]
    pub container_id: String,
    pub container_barcode: Option<String>,
}

impl InsertIntoGraph for Plate {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (rdf::type_, &cat::Plate.as_simple() as &dyn InsertIntoGraph),
            (cat::containerID, &self.container_id.as_simple() as &dyn InsertIntoGraph),
            (
                cat::containerBarcode,
                &self.container_barcode.as_ref().clone().map(|s| s.as_simple()),
            ),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Observation {
    pub value: f64,
    pub unit: Unit,
    pub error_margin: Option<ErrorMargin>,
}

/// Implementation for concrete [Observation].
impl InsertIntoGraph for Observation {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (rdf::type_, &cat::Observation.as_simple() as &dyn InsertIntoGraph),
            (qudt::unit, &self.unit.iri().as_simple() as &dyn InsertIntoGraph),
            (qudt::value, &self.value.as_simple()),
            (cat::errorMargin, &self.error_margin),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
            )?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorMargin {
    pub value: f64,
    pub unit: Unit,
}

impl InsertIntoGraph for ErrorMargin {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (rdf::type_, &cat::errorMargin.as_simple() as &dyn InsertIntoGraph),
            (qudt::unit, &self.unit.iri().as_simple() as &dyn InsertIntoGraph),
            (qudt::value, &self.value.as_simple()),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
            )?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chemical {
    #[serde(rename = "chemicalID")]
    pub chemical_id: String,
    pub chemical_name: String,
    #[serde(rename = "CASNumber")]
    pub cas_number: Option<String>,
    pub molecular_mass: Observation,
    pub smiles: String,
    pub swiss_cat_number: Option<String>,
    #[serde(rename = "Inchi")]
    pub inchi: String,
    pub keywords: Option<String>,
    pub molecular_formula: String,
    pub density: Option<Observation>,
}

impl InsertIntoGraph for Chemical {
    fn get_uri(&self) -> SimpleTerm<'static> {
        // build URI based on self.inchi
        let mut uri = cat_resource::ns.clone().as_str().to_owned();
        uri.push_str(&hash_identifier(&self.inchi));
        IriRef::new_unchecked(uri).try_into_term().unwrap()
    }

    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (rdf::type_, &obo::CHEBI_25367.as_simple() as &dyn InsertIntoGraph),
            (purl::identifier, &self.chemical_id.as_simple()),
            (allores::AFR_0002292, &self.chemical_name.as_simple()),
            (allores::AFR_0001952, &self.molecular_formula.as_simple()),
            (allores::AFR_0002295, &self.smiles.as_simple()),
            (allores::AFR_0002294, &self.molecular_mass),
            (allores::AFR_0002296, &self.inchi.as_simple()),
            (cat::casNumber, &self.cas_number.as_ref().clone().map(|s| s.as_simple())),
            (cat::swissCatNumber, &self.swiss_cat_number.as_ref().clone().map(|s| s.as_simple())),
            (schema::keywords, &self.keywords.as_ref().clone().map(|s| s.as_simple())),
            (obo::PATO_0001019, &self.density),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
            )?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeakList {
    pub peak: Vec<Peak>,
}

impl InsertIntoGraph for PeakList {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::PeakList.as_simple() as &dyn InsertIntoGraph),
            (cat::peak, &self.peak),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        let product = cat::Product.as_simple();
        let rdf_type = rdf::type_.as_simple();

        let product_object = graph.triples().filter_map(Result::ok).find_map(|[s, p, o]| {
            if *p == rdf_type && *o == product {
                Some(s.clone())
            } else {
                None
            }
        });

        if let Some(subject) = product_object {
            graph.insert(iri.clone(), cat::hasProduct.as_simple(), subject)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Peak {
    #[serde(rename = "@index")]
    pub index: i64,
    #[serde(rename = "identifier")]
    pub peak_identifier: String,
    #[serde(rename = "peak area")]
    pub peak_area: Measurement,
    #[serde(rename = "retention time")]
    pub retention_time: Measurement,
    #[serde(rename = "peak start")]
    pub peak_start: Measurement,
    #[serde(rename = "peak end")]
    pub peak_end: Measurement,
    #[serde(rename = "peak height")]
    pub peak_height: Measurement,
    #[serde(rename = "relative peak area")]
    pub relative_peak_area: Measurement,
    #[serde(rename = "relative peak height")]
    pub relative_peak_height: Measurement,
    #[serde(rename = "peak value at start")]
    pub peak_value_at_start: Measurement,
    #[serde(rename = "peak value at end")]
    pub peak_value_at_end: Measurement,
}

impl InsertIntoGraph for Peak {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &allores::AFR_0000413.as_simple() as &dyn InsertIntoGraph),
            (allores::AFR_0001164, &self.peak_identifier.as_simple()),
            (allores::AFR_0001073, &self.peak_area),
            (allores::AFR_0001089, &self.retention_time),
            (allores::AFR_0001178, &self.peak_start),
            (allores::AFR_0001180, &self.peak_end),
            (allores::AFR_0000948, &self.peak_height),
            (allores::AFR_0001165, &self.relative_peak_area),
            (allores::AFR_0000949, &self.relative_peak_height),
            (allores::AFR_0001179, &self.peak_value_at_start),
            (allores::AFR_0001181, &self.peak_value_at_end),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Measurement {
    pub value: f64,
    pub unit: Unit,
}

impl InsertIntoGraph for Measurement {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (rdf::type_, &cat::Measurement.as_simple() as &dyn InsertIntoGraph),
            (qudt::unit, &self.unit.iri().as_simple() as &dyn InsertIntoGraph),
            (qudt::value, &self.value.as_simple()),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Well {
    #[serde(flatten)]
    pub has_plate: Plate,
    pub position: String,
}

impl InsertIntoGraph for Well {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::Well.as_simple() as &dyn InsertIntoGraph),
            (cat::hasPlate, &self.has_plate),
            (allores::AFR_0002240, &self.position.as_simple()),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sophia::iri::IriRef;
    use sophia_api::term::Term;

    use crate::{
        graph::{graph_builder::GraphBuilder, insert_into::InsertIntoGraph},
        models::{ErrorMargin, Observation},
    };

    #[test]
    fn test_observation_to_triples() -> anyhow::Result<()> {
        let observation = Observation {
            value: 42.0,
            unit: Unit::DegC,
            error_margin: Some(ErrorMargin { value: 0.5, unit: Unit::DegC }),
        };

        let mut b = GraphBuilder::new();
        let i = IriRef::new_unchecked("http://test.com/my-observation");
        observation.insert_into(&mut b.graph, i.as_simple())?;

        Ok(())
    }
}
