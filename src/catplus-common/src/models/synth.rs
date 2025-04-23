use crate::{
    graph::{
        insert_into::{InsertIntoGraph, Link},
        namespaces::{alloproc, alloprop, alloqual, allores, cat, purl, qudt},
    },
    models::{
        core::{Chemical, Observation, Plate},
        enums::ActionName,
    },
};
use anyhow;
use serde::{Deserialize, Serialize};
use sophia::{
    api::ns::{rdf, xsd},
    inmem::graph::LightGraph,
};
use sophia_api::{
    graph::MutableGraph,
    term::{SimpleTerm, Term},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "Batch")]
pub struct SynthBatch {
    #[serde(rename = "batchID")]
    pub batch_id: String,
    #[serde(rename = "Actions")]
    pub actions: Vec<SynthAction>,
}

impl InsertIntoGraph for SynthBatch {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::Batch.as_simple() as &dyn InsertIntoGraph),
            (purl::identifier, &self.batch_id.as_simple()),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }

        // Loop through the actions and insert them into the Graph
        for action in &self.actions {
            if action.action_name == ActionName::AddAction {
                if let Some(wells_vector) = &action.has_well {
                    if !wells_vector.is_empty() {
                        // for every well a new Action is inserted in order to separate the Add Actions
                        // by the products they contribute to
                        for well in wells_vector {
                            let action_uri = action.get_uri();
                            graph.insert(&action_uri, cat::hasBatch.as_simple(), iri.clone())?;
                            // Create the product_id from container_id and position
                            let product_id =
                                format!("{}-{}", &well.has_plate.container_id, well.position);
                            let new_product_uri = well.get_uri();
                            for (pred, value) in [
                                (
                                    rdf::type_,
                                    &action.action_name.iri().as_simple() as &dyn InsertIntoGraph,
                                ),
                                (
                                    allores::AFX_0000622,
                                    &(action.start_time.as_str() * xsd::dateTime).as_simple(),
                                ),
                                (
                                    allores::AFR_0002423,
                                    &(action.ending_time.as_str() * xsd::dateTime).as_simple(),
                                ),
                                (allores::AFR_0001606, &action.method_name.as_simple()),
                                (allores::AFR_0001723, &action.equipment_name.as_simple()),
                                (cat::subEquipmentName, &action.sub_equipment_name.as_simple()),
                                (
                                    alloqual::AFQ_0000111,
                                    &action.dispense_state.as_ref().clone().map(|s| s.as_simple()),
                                ),
                                (
                                    cat::dispenseType,
                                    &action.dispense_type.as_ref().clone().map(|s| s.as_simple()),
                                ),
                                (cat::hasSample, &action.has_sample),
                                (qudt::quantity, &well.quantity),
                                (cat::hasWell, well),
                                (cat::producesProduct, &new_product_uri),
                            ] {
                                value.attach_into(
                                    graph,
                                    Link {
                                        source_iri: action_uri.clone(),
                                        pred: pred.as_simple(),
                                        target_iri: None,
                                    },
                                )?;
                            }
                            // Insert product that is derived from the product_id variable created above
                            for (pred, value) in [
                                (rdf::type_, &cat::Product.as_simple() as &dyn InsertIntoGraph),
                                (purl::identifier, &product_id.as_simple()),
                            ] {
                                value.attach_into(
                                    graph,
                                    Link {
                                        source_iri: new_product_uri.clone(),
                                        pred: pred.as_simple(),
                                        target_iri: None,
                                    },
                                )?;
                            }
                        }
                    }
                }
            } else {
                let action_uri = action.get_uri();
                graph.insert(&action_uri, cat::hasBatch.as_simple(), iri.clone())?;
                action.insert_into(graph, action_uri)?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "Action")]
pub struct SynthAction {
    pub action_name: ActionName,
    pub start_time: String,
    pub ending_time: String,
    pub method_name: String,
    pub equipment_name: String,
    pub sub_equipment_name: String,
    #[serde(flatten)]
    pub has_plate: Option<Plate>,
    pub speed_shaker: Option<Observation>,
    pub speed_tumble_stirrer: Option<Observation>,
    pub temperature_tumble_stirrer: Option<Observation>,
    pub temperature_shaker: Option<Observation>,
    pub pressure_measurement: Option<Observation>,
    pub vacuum: Option<Observation>,

    // These properties below are only on Synth Add Actions
    // They are entered at the Batch insert as the Add Actions
    // are multiplied by wells
    pub has_well: Option<Vec<SynthWell>>,
    pub dispense_state: Option<String>,
    pub dispense_type: Option<String>,
    pub has_sample: Option<SynthSample>,
}

impl InsertIntoGraph for SynthAction {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &self.action_name.iri().as_simple() as &dyn InsertIntoGraph),
            (allores::AFX_0000622, &(self.start_time.as_str() * xsd::dateTime).as_simple()),
            (allores::AFR_0002423, &(self.ending_time.as_str() * xsd::dateTime).as_simple()),
            (allores::AFR_0001606, &self.method_name.as_simple()),
            (allores::AFR_0001723, &self.equipment_name.as_simple()),
            (cat::subEquipmentName, &self.sub_equipment_name.as_simple()),
            (cat::speedInRPM, &self.speed_shaker),
            (cat::temperatureTumbleStirrer, &self.temperature_tumble_stirrer),
            (alloprop::AFX_0000211, &self.speed_tumble_stirrer),
            (cat::vacuum, &self.vacuum),
            (cat::temperatureShaker, &self.temperature_shaker),
            (alloproc::AFP_0002677, &self.pressure_measurement),
            (cat::hasPlate, &self.has_plate),
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
pub struct SynthWell {
    #[serde(flatten)]
    pub has_plate: Plate,
    pub position: String,
    // Quantity is added at the action not at the well
    pub quantity: Observation,
}

impl InsertIntoGraph for SynthWell {
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SynthSample {
    #[serde(flatten)]
    pub has_plate: Plate,
    #[serde(rename = "vialID")]
    pub vial_id: String,
    pub vial_type: String,
    pub role: String,
    pub expected_datum: Observation,
    pub has_sample: Vec<SampleItem>,
}

impl InsertIntoGraph for SynthSample {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (rdf::type_, &cat::Sample.as_simple() as &dyn InsertIntoGraph),
            (cat::hasPlate, &self.has_plate),
            (cat::role, &self.role.as_simple()),
            (cat::vialType, &self.vial_type.as_simple()),
            (allores::AFR_0002464, &self.vial_id.as_simple()),
            (cat::expectedDatum, &self.expected_datum),
            (cat::hasSample, &self.has_sample),
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
pub struct SampleItem {
    #[serde(rename = "sampleID")]
    pub sample_id: String,
    pub role: String,
    pub internal_bar_code: String,
    pub expected_datum: Option<Observation>,
    pub measured_quantity: Option<Observation>,
    pub concentration: Option<Observation>,
    pub physical_state: String,
    pub has_chemical: Chemical,
}

impl InsertIntoGraph for SampleItem {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (rdf::type_, &cat::Sample.as_simple() as &dyn InsertIntoGraph),
            (purl::identifier, &self.sample_id.as_simple()),
            (cat::role, &self.role.as_simple()),
            (cat::internalBarCode, &self.internal_bar_code.as_simple()),
            (alloqual::AFQ_0000111, &self.physical_state.as_simple()),
            (cat::expectedDatum, &self.expected_datum),
            (cat::measuredQuantity, &self.measured_quantity),
            (allores::AFR_0002036, &self.concentration),
            (cat::hasChemical, &self.has_chemical),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
            )?;
        }

        Ok(())
    }
}
