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
                        // for every well a new Action is inserted in order to seperate the Add Actions
                        // by the products they contribute to
                        for well in wells_vector {
                            let new_action_iri = action.get_uri();
                            graph.insert(&new_action_iri, cat::hasBatch.as_simple(), iri.clone())?;
                            graph.insert(&new_action_iri, &rdf::type_.as_simple(), &action.action_name.iri().as_simple())?;
                            graph.insert(&new_action_iri, &allores::AFX_0000622.as_simple(), &(action.start_time.as_str() * xsd::dateTime).as_simple())?;
                            graph.insert(&new_action_iri, &allores::AFR_0002423.as_simple(), &(action.ending_time.as_str() * xsd::dateTime).as_simple())?;
                            graph.insert(&new_action_iri, &allores::AFR_0001606.as_simple(), &action.method_name.as_simple())?;
                            graph.insert(&new_action_iri, &allores::AFR_0001723.as_simple(), &action.equipment_name.as_simple())?;
                            graph.insert(&new_action_iri, &cat::subEquipmentName.as_simple(), &action.sub_equipment_name.as_simple())?;
                            action.has_sample.attach_into(graph, Link { source_iri: new_action_iri.clone(), pred: cat::hasSample.as_simple(), target_iri: None })?;
                            if let Some(dispense_type) = &action.dispense_type {
                                graph.insert(&new_action_iri, &cat::dispenseType.as_simple(), dispense_type.as_simple())?;
                            }
                            if let Some(dispense_state) = &action.dispense_state {
                                graph.insert(&new_action_iri, &alloqual::AFQ_0000111.as_simple(), dispense_state.as_simple())?;
                            }
                            well.quantity.attach_into(graph, Link { source_iri: new_action_iri.clone(), pred: qudt::quantity.as_simple(), target_iri: None })?;
                            well.attach_into(graph, Link { source_iri: new_action_iri.clone(), pred: cat::hasWell.as_simple(), target_iri: None })?;
                            let product_id = format!("{}-{}", &well.has_plate.container_id, well.position);
                            let new_product_iri = well.get_uri();
                            graph.insert(&new_product_iri, &rdf::type_.as_simple(), &cat::Product.as_simple())?;
                            graph.insert(&new_product_iri, &purl::identifier.as_simple(), product_id.as_simple())?;
                            graph.insert(&new_action_iri, &cat::producesProduct.as_simple(), &new_product_iri)?;
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
