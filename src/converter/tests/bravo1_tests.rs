use catplus_common::{models::bravo::BravoActionWrapper, rdf::rdf_parser::parse_turtle_to_graph};
use converter::convert::{json_to_rdf, RdfFormat};
use sophia_isomorphism::isomorphic_graphs;
use std::path::Path;

mod common;
use common::get_test_config;

#[test]
fn test_convert_bravo1_add_action() {
    let config = get_test_config("data/tests/bravo1_add_action.json");
    let result = json_to_rdf::<BravoActionWrapper>(&config);
    let expected_ttl = r#"
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
        PREFIX cat: <http://example.org/cat#>
        PREFIX schema: <https://schema.org/>
        PREFIX unit: <http://qudt.org/vocab/unit/>
        PREFIX allores: <http://purl.allotrope.org/ontologies/result#>
        PREFIX allorole: <http://purl.allotrope.org/ontologies/role#>
        PREFIX alloproc: <http://purl.allotrope.org/ontologies/process#>
        PREFIX alloprop: <http://purl.allotrope.org/ontologies/property#>
        PREFIX allocom: <http://purl.allotrope.org/ontologies/common#>
        PREFIX allohdf: <http://purl.allotrope.org/ontologies/hdf5/1.8#>
        PREFIX allohdfcube: <http://purl.allotrope.org/ontologies/datacube-hdf-map#>
        PREFIX qb: <http://purl.org/linked-data/cube#>
        PREFIX qudt: <http://qudt.org/schema/qudt/>
        PREFIX qudtext: <http://purl.allotrope.org/ontology/qudt-ext/unit#>
        PREFIX alloqual: <http://purl.allotrope.org/ontologies/quality#>
        PREFIX allodc: <http://purl.allotrope.org/ontologies/datacube#>
        PREFIX purl: <http://purl.allotrope.org/ontologies/>
        PREFIX obo: <http://purl.obolibrary.org/obo/>

        [] a cat:AddAction;
        cat:dispenseType "volume";
        cat:hasSolvent [ a cat:Solvent;
            cat:hasChemical [ a obo:CHEBI_25367;
                cat:casNumber "7732-18-5";
                cat:swissCatNumber "SwissCAT-962";
                purl:identifier "14";
                allores:AFR_0001952 "H2O";
                allores:AFR_0002292 "Water";
                allores:AFR_0002294 [ a cat:Observation;
                    qudt:unit unit:GM-PER-MOL;
                    qudt:value "18.015"^^xsd:double];
                allores:AFR_0002295 "O";
                allores:AFR_0002296 "1S/H2O/h1H2";
                obo:PATO_0001019 [ a cat:Observation;
                    qudt:unit unit:GM-PER-MilliL;
                    qudt:value "1"^^xsd:double]];
            cat:volume [ a cat:Observation;
                cat:errorMargin [ a cat:errorMargin;
                    qudt:unit unit:MilliL;
                    qudt:value "0.01"^^xsd:double];
                qudt:unit unit:MilliL;
                qudt:value "0.5"^^xsd:double]];
        cat:order "2";
        cat:preparesProduct [ a cat:Product;
            purl:identifier "1-A1"];
        alloqual:AFQ_0000111 "Liquid";
        allores:AFR_0001164 "511359d7-df0d-4018-bfee-ff58585b5809";
        allores:AFR_0001606 "DilutionAddAction";
        allores:AFR_0001723 "Micropipette";
        allores:AFR_0002423 "2024-07-25T12:02:41"^^xsd:dateTime;
        allores:AFX_0000622 "2024-07-25T12:02:39"^^xsd:dateTime.
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}

#[test]
fn test_convert_bravo1_evaporation_action() {
    let config = get_test_config("data/tests/bravo1_evaporation_action.json");
    let result = json_to_rdf::<BravoActionWrapper>(&config);
    let expected_ttl = r#"
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
        PREFIX cat: <http://example.org/cat#>
        PREFIX schema: <https://schema.org/>
        PREFIX unit: <http://qudt.org/vocab/unit/>
        PREFIX allores: <http://purl.allotrope.org/ontologies/result#>
        PREFIX allorole: <http://purl.allotrope.org/ontologies/role#>
        PREFIX alloproc: <http://purl.allotrope.org/ontologies/process#>
        PREFIX alloprop: <http://purl.allotrope.org/ontologies/property#>
        PREFIX allocom: <http://purl.allotrope.org/ontologies/common#>
        PREFIX allohdf: <http://purl.allotrope.org/ontologies/hdf5/1.8#>
        PREFIX allohdfcube: <http://purl.allotrope.org/ontologies/datacube-hdf-map#>
        PREFIX qb: <http://purl.org/linked-data/cube#>
        PREFIX qudt: <http://qudt.org/schema/qudt/>
        PREFIX qudtext: <http://purl.allotrope.org/ontology/qudt-ext/unit#>
        PREFIX alloqual: <http://purl.allotrope.org/ontologies/quality#>
        PREFIX allodc: <http://purl.allotrope.org/ontologies/datacube#>
        PREFIX purl: <http://purl.allotrope.org/ontologies/>
        PREFIX obo: <http://purl.obolibrary.org/obo/>

        [] a cat:EvaporationAction;
        cat:hasWell [ a cat:Well;
            cat:hasPlate [ a cat:Plate;
                cat:containerBarcode "1234858858754848";
                cat:containerID "157"];
            allores:AFR_0002240 "A1"];
        cat:order "1";
        cat:preparesProduct [ a cat:Product;
            purl:identifier "1-A1"];
        cat:subEquipmentName "item-1";
        cat:volumeEvaporationFinal [ a cat:Observation;
            qudt:unit unit:PERCENT;
            qudt:value "50"^^xsd:double];
        alloprop:AFX_0000060 [ a cat:Observation;
            cat:errorMargin [ a cat:errorMargin;
                qudt:unit unit:DEG_C;
                qudt:value "1"^^xsd:double];
            qudt:unit unit:DEG_C;
            qudt:value "156"^^xsd:double];
        allores:AFR_0001164 "511359d7-df0d-4018-bfee-ff58585b5809";
        allores:AFR_0001606 "Evaporate";
        allores:AFR_0001723 "Evaporator";
        allores:AFR_0002423 "2024-07-25T12:15:20"^^xsd:dateTime;
        allores:AFX_0000622 "2024-07-25T12:03:31"^^xsd:dateTime.
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}

#[test]
fn test_convert_bravo1_solvent_change_action() {
    let config = get_test_config("data/tests/bravo1_solvent_change_action.json");
    let result = json_to_rdf::<BravoActionWrapper>(&config);
    let expected_ttl = r#"
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
        PREFIX cat: <http://example.org/cat#>
        PREFIX schema: <https://schema.org/>
        PREFIX unit: <http://qudt.org/vocab/unit/>
        PREFIX allores: <http://purl.allotrope.org/ontologies/result#>
        PREFIX allorole: <http://purl.allotrope.org/ontologies/role#>
        PREFIX alloproc: <http://purl.allotrope.org/ontologies/process#>
        PREFIX alloprop: <http://purl.allotrope.org/ontologies/property#>
        PREFIX allocom: <http://purl.allotrope.org/ontologies/common#>
        PREFIX allohdf: <http://purl.allotrope.org/ontologies/hdf5/1.8#>
        PREFIX allohdfcube: <http://purl.allotrope.org/ontologies/datacube-hdf-map#>
        PREFIX qb: <http://purl.org/linked-data/cube#>
        PREFIX qudt: <http://qudt.org/schema/qudt/>
        PREFIX qudtext: <http://purl.allotrope.org/ontology/qudt-ext/unit#>
        PREFIX alloqual: <http://purl.allotrope.org/ontologies/quality#>
        PREFIX allodc: <http://purl.allotrope.org/ontologies/datacube#>
        PREFIX purl: <http://purl.allotrope.org/ontologies/>
        PREFIX obo: <http://purl.obolibrary.org/obo/>

        [] a cat:SolventChangeAction;
        cat:endingDuration [ a cat:Observation;
            qudt:unit unit:MIN;
            qudt:value "1"^^xsd:double];
        cat:hasCartridge [ a cat:Cartridge;
            cat:cartridgeComposition "test-material";
            cat:cartridgeName "test-cartridge"];
        cat:hasSolvent [ a cat:Solvent;
            cat:hasChemical [ a obo:CHEBI_25367;
                cat:casNumber "75-05-8";
                cat:swissCatNumber "SwissCAT-6342";
                purl:identifier "25";
                allores:AFR_0001952 "C2H3N";
                allores:AFR_0002292 "Acetonitrile";
                allores:AFR_0002294 [ a cat:Observation;
                    qudt:unit unit:GM-PER-MOL;
                    qudt:value "41.05"^^xsd:double];
                allores:AFR_0002295 "CC#N";
                allores:AFR_0002296 "1S/C2H3N/c1-2-3/h1H3";
                obo:PATO_0001019 [ a cat:Observation;
                    qudt:unit unit:GM-PER-MilliL;
                    qudt:value "0.787"^^xsd:double]];
            cat:volume [ a cat:Observation;
                cat:errorMargin [ a cat:errorMargin;
                    qudt:unit unit:MicroL;
                    qudt:value "0.5"^^xsd:double];
                qudt:unit unit:MicroL;
                qudt:value "50"^^xsd:double]];
        cat:isSpmeProcess true;
        cat:order "3";
        cat:preparesProduct [ a cat:Product;
            purl:identifier "1-A1"];
        cat:startDuration [ a cat:Observation;
            qudt:unit unit:MIN;
            qudt:value "0"^^xsd:double];
        cat:subEquipmentName "cartridge exchange";
        allores:AFR_0001164 "511359d7-df0d-4018-bfee-ff58585b5809";
        allores:AFR_0001606 "separation-cartridge-part-1";
        allores:AFR_0001723 "SPE";
        allores:AFR_0002423 "2024-07-25T12:15:20"^^xsd:dateTime;
        allores:AFX_0000622 "2024-07-25T12:03:31"^^xsd:dateTime.
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}
