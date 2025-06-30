use catplus_common::{models::bravo::BravoActionWrapper, rdf::rdf_parser::parse_turtle_to_graph};
use converter::convert::json_to_rdf;
use sophia_isomorphism::isomorphic_graphs;

mod common;
use common::get_test_config;

#[test]
fn test_convert_bravo2_add_action() {
    let config = get_test_config("data/tests/bravo2_add_action.json");
    let result = json_to_rdf::<BravoActionWrapper>(&config);
    let expected_ttl = r#"
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
        PREFIX cat: <http://example.org/catplus/ontology/>
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

        <http://example.org/cat/resource/o6CWfo3Pzp4EDQmJYTfgqzeNia1nVosWTDBFFotwFts> a obo:CHEBI_25367;
        cat:casNumber "811-98-3";
        cat:swissCatNumber "SwissCAT-71568";
        purl:identifier "25";
        allores:AFR_0001952 "CH4O";
        allores:AFR_0002292 "Tetradeuteromethanol";
        allores:AFR_0002294 [ a cat:Observation;
            qudt:unit unit:GM-PER-MOL;
            qudt:value "36.07"^^xsd:double];
        allores:AFR_0002295 "[2H]C([2H])([2H])O[2H]";
        allores:AFR_0002296 "1S/CH4O/c1-2/h2H,1H3/i1D3,2D";
        obo:PATO_0001019 [ a cat:Observation;
            qudt:unit unit:GM-PER-MilliL;
            qudt:value "0.89"^^xsd:double].

        <http://example.org/cat/resource/tFZK0UBWAzCoe3VPYQ1NgSFz1q-ziFOVQmJE_XWRckw> a cat:Product;
        purl:identifier "1-A1".

        [] a cat:BravoAddAction;
        cat:dispenseType "volume";
        cat:hasSolvent [ a cat:Solvent;
            cat:hasChemical <http://example.org/cat/resource/o6CWfo3Pzp4EDQmJYTfgqzeNia1nVosWTDBFFotwFts>;
            cat:volume [ a cat:Observation;
                cat:errorMargin [ a cat:errorMargin;
                    qudt:unit unit:MilliL;
                    qudt:value "0.01"^^xsd:double];
                qudt:unit unit:MilliL;
                qudt:value "0.2"^^xsd:double]];
        cat:order "3";
        cat:preparesProduct <http://example.org/cat/resource/tFZK0UBWAzCoe3VPYQ1NgSFz1q-ziFOVQmJE_XWRckw>;
        alloqual:AFQ_0000111 "Liquid";
        allores:AFR_0001164 "511359d7-df0d-4018-bfee-ff58585b5809";
        allores:AFR_0001606 "DissolutionAddAction";
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
fn test_convert_bravo2_evaporation_action() {
    let config = get_test_config("data/tests/bravo2_evaporation_action.json");
    let result = json_to_rdf::<BravoActionWrapper>(&config);
    let expected_ttl = r#"
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
        PREFIX cat: <http://example.org/catplus/ontology/>
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

        <http://example.org/cat/resource/tFZK0UBWAzCoe3VPYQ1NgSFz1q-ziFOVQmJE_XWRckw> a cat:Product;
        purl:identifier "1-A1".

        [] a cat:EvaporationAction;
        cat:order "1";
        cat:preparesProduct <http://example.org/cat/resource/tFZK0UBWAzCoe3VPYQ1NgSFz1q-ziFOVQmJE_XWRckw>;
        cat:subEquipmentName "item-1";
        cat:volumeEvaporationFinal [ a cat:Observation;
            qudt:unit unit:PERCENT;
            qudt:value "100"^^xsd:double];
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
fn test_convert_bravo2_shake_action() {
    let config = get_test_config("data/tests/bravo2_shake_action.json");
    let result = json_to_rdf::<BravoActionWrapper>(&config);
    let expected_ttl = r#"
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
        PREFIX cat: <http://example.org/catplus/ontology/>
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

        <http://example.org/cat/resource/tFZK0UBWAzCoe3VPYQ1NgSFz1q-ziFOVQmJE_XWRckw> a cat:Product;
        purl:identifier "1-A1".

        [] a cat:ShakeAction;
        cat:order "5";
        cat:preparesProduct <http://example.org/cat/resource/tFZK0UBWAzCoe3VPYQ1NgSFz1q-ziFOVQmJE_XWRckw>;
        cat:speedInRPM [ a cat:Observation;
            cat:errorMargin [ a cat:errorMargin;
                qudt:unit unit:REV-PER-MIN;
                qudt:value "1"^^xsd:double];
            qudt:unit unit:REV-PER-MIN;
            qudt:value "152"^^xsd:double];
        cat:subEquipmentName "item-1";
        allores:AFR_0001164 "511359d7-df0d-4018-bfee-ff58585b5809";
        allores:AFR_0001606 "shake";
        allores:AFR_0001723 "magneticStirrer";
        allores:AFR_0002423 "2024-07-25T12:15:20"^^xsd:dateTime;
        allores:AFX_0000622 "2024-07-25T12:03:31"^^xsd:dateTime.
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}
