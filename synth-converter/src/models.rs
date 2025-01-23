use std::intrinsics::mir::mir;
use std::fmt;

// The structure follows the input data as descibed in the
// https://github.com/sdsc-ordes/cat-plus-ontology see here for the expected Synth input data:
// https://github.com/sdsc-ordes/cat-plus-ontology/tree/96091fd2e75e03de8a4c4d66ad502b2db27998bd/json-file/1-Synth
use anyhow;
use serde::{Deserialize, Serialize};
use crate::graph::{
    namespaces::{alloqual, allores, alloproc, cat, obo, purl, qudt, schema},
    utils::generate_bnode_term,
};
use sophia_api::{
    ns::NsTerm,
    term::{FromTerm, SimpleTerm, Term},
};
use sophia::{
    api::{
        graph::MutableGraph,
        ns::{rdf, xsd},
    },
    inmem::graph::LightGraph,
};

pub fn link_node<N>(source_uri: SimpleTerm, predicate: SimpleTerm, node: N) -> Vec<[SimpleTerm; 3]> 
    where N: ToGraph {
    let node_uri = node.get_uri();
    let mut triples = vec![[source_uri.clone(), predicate.clone(), node_uri.clone()]];
    triples.append(&mut node.to_triples(node_uri.clone()));

    triples
}

pub trait ToGraph {
    fn to_triples(&self, subject: SimpleTerm) -> Vec<[SimpleTerm; 3]>;

    fn to_graph(&self, subject: SimpleTerm) -> anyhow::Result<LightGraph> {
        let mut graph = LightGraph::new();
        let triples = self.to_triples(subject);
        for triple in triples {
            graph.insert(&triple[0], &triple[1], &triple[2])?;
        }
        return Ok(graph)
    }

    fn get_uri(&self) -> SimpleTerm<'static> { 
      generate_bnode_term()
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Batch {
    #[serde(rename = "batchID")]
    pub batch_id: String,
    #[serde(rename = "Actions")]
    pub actions: Vec<Action>,
    pub batch_name: Option<String>,
    #[serde(rename = "ReactionType")]
    pub reaction_type: Option<String>,
    #[serde(rename = "OptimizationType")]
    pub optimization_type: Option<String>,
    #[serde(rename = "Link")]
    pub link: Option<String>,
}

impl ToGraph for Batch {
    fn to_triples(&self, subject: SimpleTerm) -> Vec<[SimpleTerm; 3]> {
        let mut triples: Vec<[SimpleTerm; 3]> = [
            (&rdf::type_, &cat::Batch),
            (&schema::name, &self::batch_id),
        ]
            .into_iter().map(|(predicate, object)| {
                [subject.clone(), predicate.as_simple(), object.as_simple()]
        
            })
            .collect();

        for action in &self.actions {
            let action_subject = action.get_uri();
            triples.push([action_subject, cat::hasBatch.as_simple(), subject.clone()]);

            triples.append(&mut action.to_triples(action_subject));
        }

        triples
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub action_name: ActionName,
    pub start_time: String,
    pub ending_time: String,
    pub method_name: String,
    pub equipment_name: String,
    pub sub_equipment_name: String,
    #[serde(flatten)]
    pub container_info: Option<ContainerInfo>,
    pub speed_shaker: Option<Observation>,
    pub has_container_position_and_quantity: Option<Vec<ContainerPositionQuantityItem>>,
    pub dispense_state: Option<String>,
    pub dispense_type: Option<String>,
    pub has_sample: Option<Sample>,
    pub speed_tumble_stirrer: Option<Observation>,
    pub temperature_tumble_stirrer: Option<Observation>,
    pub temperature_shaker: Option<Observation>,
    pub pressure_measurement: Option<Observation>,
}

impl ToGraph for Action {
    fn to_triples(&self, subject: SimpleTerm) -> Vec<[SimpleTerm; 3]> {
        let subject: SimpleTerm = generate_bnode_term();

        // Data properties.
        let data_properties = vec![
            (&allores::AFX_0000622,  &self.start_time),
            (&allores::AFR_0002423,  &self.ending_time),
            (&allores::AFR_0001606,  &self.method_name),
            (&allores::AFR_0001723,  &self.equipment_name),
            (&cat::subEquipmentName, &self.sub_equipment_name),
            (&rdf::type_, &self.action_name.to_string()),
        ];

        if let Some(dispense_type) = &self.dispense_type {
            data_properties.push((&cat::dispenseType, dispense_type));
        }

        // Object properties
        let object_properties: Vec<(NsTerm, Option<&dyn ToGraph>)> = vec![
            (cat::temperatureShakerShape, self.temperature_shaker),
            (cat::temperatureTumbleStirrerShape, self.temperature_tumble_stirrer),
            (cat::speedInRPM, self.speed_shaker),
            (cat::speedTumbleStirrerShape, self.speed_tumble_stirrer),
            (alloproc::AFP_0002677, self.pressure_measurement),
        ];

        if let Some(container_pos) = &self.has_container_position_and_quantity {
            for container_item in container_pos {
                object_properties.push(
                    (cat::hasContainerPositionAndQuantity, Some(container_item)),
                )
            }
        }

        // Generate triples.
        let mut triples: Vec<[SimpleTerm; 3]> = data_properties
            .into_iter().map(|(predicate, object)| {
                [subject.clone(), predicate.as_simple(), object.as_simple()]
        
            })
            .collect();

        for (pred, object) in object_properties {
            if let Some(obj) = object {
                triples.append(&mut link_node(
                    subject.clone(),
                    pred.as_simple(),
                    obj,
                ));
            }
        }

        // NOTE: for container_info, we attach triples directly to the action
        if let Some(container_info) = &self.container_info {
            triples.append(container_info.to_triples(subject.clone()));
        };


        if let Some(sample) = &self.has_sample {
            self.insert_samples(&action_term, sample)?;
        }
        
        triples
    }

}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case, non_camel_case_types)]
pub enum ActionName {
    AddAction,
    setTemperatureAction,
    filtrateAction,
    shakeAction,
    setVacuumAction,
    setPressureAction,
}

impl fmt::Display for ActionName {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let term = match self {
            Self::AddAction => cat::AddAction,
            Self::setTemperatureAction => cat::SetTemperatureAction,
            Self::setPressureAction => cat::SetPressureAction,
            Self::shakeAction => cat::ShakeAction,
            Self::setVacuumAction => cat::SetVacuumAction,
            Self::filtrateAction => cat::FiltrateAction,
        };

        write!(f, "{}", term.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerInfo {
    #[serde(rename = "containerID")]
    pub container_id: String,
    pub container_barcode: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Observation {
    pub value: f64,
    pub unit: String,
    pub error_margin: Option<ErrorMargin>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMargin {
    pub value: f64,
    pub unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sample {
    #[serde(flatten)]
    pub container: ContainerInfo,
    #[serde(rename = "vialID")]
    pub vial_id: String,
    pub vial_type: String,
    pub role: String,
    pub expected_datum: Observation,
    pub has_sample: Vec<SampleItem>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerPositionQuantityItem {
    #[serde(rename = "containerID")]
    pub container_id: String,
    pub position: String,
    pub quantity: Observation,
}
