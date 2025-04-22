use catplus_common::{models::bravo::BravoBatch, rdf::rdf_parser::parse_turtle_to_graph};
use converter::convert::{json_to_rdf, RdfFormat};
use sophia_isomorphism::isomorphic_graphs;

#[test]
fn test_convert_bravo2_add_action() {
    let output_format = RdfFormat::Turtle;
    let json_data = r#"
        {
            "Actions": [
                {
                    "actionName": "AddAction",
                    "methodName": "DissolutionAddAction",
                    "equipmentName": "Micropipette",
                    "startTime": "2024-07-25T12:02:39",
                    "endingTime": "2024-07-25T12:02:41",
                    "dispenseState": "Liquid",
                    "dispenseType": "volume",
                    "productIdentification": {
                        "sampleID": "1-A1",
                        "peakIdentifier": "511359d7-df0d-4018-bfee-ff58585b5809"
                    },
                    "hasWell": {
                        "containerID": "157",
                        "containerBarcode": "1234858858754848",
                        "position": "A1"
                    },
                    "hasSolvent": {
                        "hasChemical": {
                            "chemicalID": "25",
                            "chemicalName": "Tetradeuteromethanol",
                            "CASNumber": "811-98-3",
                            "molecularMass": {
                                "value": 36.07,
                                "unit": "g/mol"
                            },
                            "smiles": "[2H]C([2H])([2H])O[2H]",
                            "swissCatNumber": "SwissCAT-71568",
                            "Inchi": "1S/CH4O/c1-2/h2H,1H3/i1D3,2D",
                            "molecularFormula": "CH4O",
                            "density": {
                                "value": 0.89,
                                "unit": "g/mL"
                            }
                        },
                        "volume": {
                            "value": 0.2,
                            "unit": "mL",
                            "errorMargin": {
                                "value": 0.01,
                                "unit": "mL"
                            }
                        }
                    },
                    "order": "3"
                }
            ]
        }
    "#;
    let result = json_to_rdf::<BravoBatch>(json_data, &output_format, false);
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
                    qudt:value "0.89"^^xsd:double]];
            cat:volume [ a cat:Observation;
                cat:errorMargin [ a cat:errorMargin;
                    qudt:unit unit:MilliL;
                    qudt:value "0.01"^^xsd:double];
                qudt:unit unit:MilliL;
                qudt:value "0.2"^^xsd:double]];
        cat:order "3";
        cat:preparesProduct [ a cat:Product;
            purl:identifier "1-A1"];
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
    let output_format = RdfFormat::Turtle;
    let json_data = r#"
        {
            "Actions": [
                {
                    "actionName": "EvaporationAction",
                    "methodName": "Evaporate",
                    "equipmentName": "Evaporator",
                    "subEquipmentName": "item-1",
                    "volumeEvaporationFinal": {
                        "value": 100,
                        "unit": "%"
                    },
                    "startTime": "2024-07-25T12:03:31",
                    "endingTime": "2024-07-25T12:15:20",
                    "productIdentification": {
                        "sampleID": "1-A1",
                        "peakIdentifier": "511359d7-df0d-4018-bfee-ff58585b5809"
                    },
                    "hasWell": {
                        "containerID": "157",
                        "containerBarcode": "1234858858754848",
                        "position": "A1"
                    },
                    "temperature": {
                        "value": 156,
                        "unit": "°C",
                        "errorMargin": {
                            "value": 1,
                            "unit": "°C"
                        }
                    },
                    "order": "1"
                }
            ]
        }
    "#;
    let result = json_to_rdf::<BravoBatch>(json_data, &output_format, false);
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
        cat:order "1";
        cat:preparesProduct [ a cat:Product;
            purl:identifier "1-A1"];
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
    let output_format = RdfFormat::Turtle;
    let json_data = r#"
        {
            "Actions": [
                {
                    "actionName": "shakeAction",
                    "methodName": "shake",
                    "equipmentName": "magneticStirrer",
                    "subEquipmentName": "item-1",
                    "startTime": "2024-07-25T12:03:31",
                    "endingTime": "2024-07-25T12:15:20",
                    "containerID": "157",
                    "containerBarcode": "1234858858754848",
                    "productIdentification": {
                        "sampleID": "1-A1",
                        "peakIdentifier": "511359d7-df0d-4018-bfee-ff58585b5809"
                    },
                    "speedShaker": {
                        "value": 152,
                        "unit": "rpm",
                        "errorMargin": {
                            "value": 1,
                            "unit": "rpm"
                        }
                    },
                    "order": "5"
                }
            ]
        }
    "#;
    let result = json_to_rdf::<BravoBatch>(json_data, &output_format, false);
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

        [] a cat:ShakeAction;
        cat:order "5";
        cat:preparesProduct [ a cat:Product;
            purl:identifier "1-A1"];
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
