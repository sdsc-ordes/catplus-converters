use catplus_common::{models::synth::SynthBatch, rdf::rdf_parser::parse_turtle_to_graph};
use converter::convert::json_to_rdf;
use sophia_isomorphism::isomorphic_graphs;

mod common;
use common::get_test_config;

#[test]
fn test_convert_filtrate_action() {
    let config = get_test_config("data/tests/synth_filtrate_action.json");
    let result = json_to_rdf::<SynthBatch>(&config);
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

        <http://example.org/cat/resource/U1-jDX4l3YpJ8VNneXNOyChhCNEV2lBF1387QYXY95A> a cat:Batch;
          purl:identifier "23".

        [] a cat:FiltrateAction;
          cat:hasBatch <http://example.org/cat/resource/U1-jDX4l3YpJ8VNneXNOyChhCNEV2lBF1387QYXY95A>;
          cat:hasPlate [ a cat:Plate;
              cat:containerBarcode "1";
              cat:containerID "1"];
          cat:subEquipmentName "Filtration unit";
          allores:AFR_0001606 "filtrate";
          allores:AFR_0001723 "Chemspeed SWING XL";
          allores:AFR_0002423 "2024-07-25T12:16:50"^^xsd:dateTime;
          allores:AFX_0000622 "2024-07-25T12:15:23"^^xsd:dateTime.
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}

#[test]
fn test_convert_pressure_action() {
    let config = get_test_config("data/tests/synth_set_pressure_action.json");
    let result = json_to_rdf::<SynthBatch>(&config);
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

        <http://example.org/cat/resource/U1-jDX4l3YpJ8VNneXNOyChhCNEV2lBF1387QYXY95A> a cat:Batch;
          purl:identifier "23".

        [] a cat:SetPressureAction;
          cat:hasBatch <http://example.org/cat/resource/U1-jDX4l3YpJ8VNneXNOyChhCNEV2lBF1387QYXY95A>;
          cat:hasPlate [ a cat:Plate;
              cat:containerBarcode "1";
              cat:containerID "1"];
          cat:subEquipmentName "MTP_Pressure";
          alloproc:AFP_0002677 [ a cat:Observation;
              cat:errorMargin [ a cat:errorMargin;
                  qudt:unit unit:Bar;
                  qudt:value "1"^^xsd:double];
              qudt:unit unit:Bar;
              qudt:value "5"^^xsd:double];
          allores:AFR_0001606 "set_pressure";
          allores:AFR_0001723 "Chemspeed SWING XL";
          allores:AFR_0002423 "2024-07-25T12:04:05"^^xsd:dateTime;
          allores:AFX_0000622 "2024-07-25T12:03:50"^^xsd:dateTime.
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}

#[test]
fn test_convert_set_temperature_action() {
    let config = get_test_config("data/tests/synth_set_temperature_action.json");
    let result = json_to_rdf::<SynthBatch>(&config);
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

        <http://example.org/cat/resource/U1-jDX4l3YpJ8VNneXNOyChhCNEV2lBF1387QYXY95A> a cat:Batch;
        purl:identifier "23".

        [] a cat:SetTemperatureAction;
        cat:hasBatch <http://example.org/cat/resource/U1-jDX4l3YpJ8VNneXNOyChhCNEV2lBF1387QYXY95A>;
        cat:hasPlate [ a cat:Plate;
            cat:containerBarcode "1";
            cat:containerID "1"];
        cat:subEquipmentName "heater";
        cat:temperatureShaker [ a cat:Observation;
            cat:errorMargin [ a cat:errorMargin;
                qudt:unit unit:DEG_C;
                qudt:value "1"^^xsd:double];
            qudt:unit unit:DEG_C;
            qudt:value "25"^^xsd:double];
        cat:temperatureTumbleStirrer [ a cat:Observation;
            cat:errorMargin [ a cat:errorMargin;
                qudt:unit unit:DEG_C;
                qudt:value "2"^^xsd:double];
            qudt:unit unit:DEG_C;
            qudt:value "25"^^xsd:double];
        alloprop:AFX_0000211 [ a cat:Observation;
            cat:errorMargin [ a cat:errorMargin;
                qudt:unit unit:REV-PER-MIN;
                qudt:value "5"^^xsd:double];
            qudt:unit unit:REV-PER-MIN;
            qudt:value "152"^^xsd:double];
        allores:AFR_0001606 "set_temperature";
        allores:AFR_0001723 "Chemspeed SWING XL";
        allores:AFR_0002423 "2024-07-25T12:00:02"^^xsd:dateTime;
        allores:AFX_0000622 "2024-07-25T12:00:00"^^xsd:dateTime.
        "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}

#[test]
fn test_convert_add_action() {
    let config = get_test_config("data/tests/synth_add_action.json");
    let result = json_to_rdf::<SynthBatch>(&config);
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

        <http://example.org/cat/resource/U1-jDX4l3YpJ8VNneXNOyChhCNEV2lBF1387QYXY95A> a cat:Batch;
        purl:identifier "23".

        <http://example.org/cat/resource/cK09bQkyv-c1OdUpG17RCAzSw1k_Z5EP9db8jh2WTvg> a obo:CHEBI_25367;
        cat:casNumber "123-11-5";
        purl:identifier "134";
        allores:AFR_0001952 "C8H8O2";
        allores:AFR_0002292 "4-methoxybenzaldehyde";
        allores:AFR_0002294 [ a cat:Observation;
            qudt:unit unit:GM-PER-MOL;
            qudt:value "136.15"^^xsd:double],
            [ a cat:Observation;
            qudt:unit unit:GM-PER-MOL;
            qudt:value "136.15"^^xsd:double];
        allores:AFR_0002295 "COC1=CC=C(C=C1)C=O";
        allores:AFR_0002296 "1S/C8H8O2/c1-10-8-4-2-7(6-9)3-5-8/h2-6H,1H3";
        obo:PATO_0001019 [ a cat:Observation;
            qudt:unit unit:GM-PER-MilliL;
            qudt:value "1.119"^^xsd:double],
            [ a cat:Observation;
            qudt:unit unit:GM-PER-MilliL;
            qudt:value "1.119"^^xsd:double].

        <http://example.org/cat/resource/rIm5C_G7z1IMzoHcbRYk4DGARaf09abUq6oKKwym2oU> a cat:Product;
        purl:identifier "1-B1".

        <http://example.org/cat/resource/tFZK0UBWAzCoe3VPYQ1NgSFz1q-ziFOVQmJE_XWRckw> a cat:Product;
        purl:identifier "1-A1".

        [] a cat:SynthAddAction;
        cat:dispenseType "volume";
        cat:hasBatch <http://example.org/cat/resource/U1-jDX4l3YpJ8VNneXNOyChhCNEV2lBF1387QYXY95A>;
        cat:hasSample [ a cat:Sample;
            cat:expectedDatum [ a cat:Observation;
                qudt:unit unit:MilliGM;
                qudt:value "2"^^xsd:double];
            cat:hasPlate [ a cat:Plate;
                cat:containerBarcode "18";
                cat:containerID "18"];
            cat:hasSample [ a cat:Sample;
                cat:expectedDatum [ a cat:Observation;
                    qudt:unit unit:MilliGM;
                    qudt:value "5"^^xsd:double];
                cat:hasChemical <http://example.org/cat/resource/cK09bQkyv-c1OdUpG17RCAzSw1k_Z5EP9db8jh2WTvg>;
                cat:internalBarCode "2";
                cat:measuredQuantity [ a cat:Observation;
                    cat:errorMargin [ a cat:errorMargin;
                        qudt:unit unit:MilliGM;
                        qudt:value "0.001"^^xsd:double];
                    qudt:unit unit:MilliGM;
                    qudt:value "1"^^xsd:double];
                cat:role "reagent";
                purl:identifier "124";
                alloqual:AFQ_0000111 "Liquid"];
            cat:role "reagent";
            cat:vialType "storage vial";
            allores:AFR_0002464 "17"];
        cat:hasWell [ a cat:Well;
            cat:hasPlate [ a cat:Plate;
                cat:containerID "1"];
            allores:AFR_0002240 "B1"];
        cat:producesProduct <http://example.org/cat/resource/rIm5C_G7z1IMzoHcbRYk4DGARaf09abUq6oKKwym2oU>;
        cat:subEquipmentName "GDU-V";
        alloqual:AFQ_0000111 "Liquid";
        allores:AFR_0001606 "addition";
        allores:AFR_0001723 "Chemspeed SWING XL";
        allores:AFR_0002423 "2024-07-25T12:01:35"^^xsd:dateTime;
        allores:AFX_0000622 "2024-07-25T12:01:29"^^xsd:dateTime;
        qudt:quantity [ a cat:Observation;
            cat:errorMargin [ a cat:errorMargin;
                qudt:unit unit:MilliGM;
                qudt:value "0.002"^^xsd:double];
            qudt:unit unit:MilliGM;
            qudt:value "0.034"^^xsd:double].

        [] a cat:SynthAddAction;
        cat:dispenseType "volume";
        cat:hasBatch <http://example.org/cat/resource/U1-jDX4l3YpJ8VNneXNOyChhCNEV2lBF1387QYXY95A>;
        cat:hasSample [ a cat:Sample;
            cat:expectedDatum [ a cat:Observation;
                qudt:unit unit:MilliGM;
                qudt:value "2"^^xsd:double];
            cat:hasPlate [ a cat:Plate;
                cat:containerBarcode "18";
                cat:containerID "18"];
            cat:hasSample [ a cat:Sample;
                cat:expectedDatum [ a cat:Observation;
                    qudt:unit unit:MilliGM;
                    qudt:value "5"^^xsd:double];
                cat:hasChemical <http://example.org/cat/resource/cK09bQkyv-c1OdUpG17RCAzSw1k_Z5EP9db8jh2WTvg>;
                cat:internalBarCode "2";
                cat:measuredQuantity [ a cat:Observation;
                    cat:errorMargin [ a cat:errorMargin;
                        qudt:unit unit:MilliGM;
                        qudt:value "0.001"^^xsd:double];
                    qudt:unit unit:MilliGM;
                    qudt:value "1"^^xsd:double];
                cat:role "reagent";
                purl:identifier "124";
                alloqual:AFQ_0000111 "Liquid"];
            cat:role "reagent";
            cat:vialType "storage vial";
            allores:AFR_0002464 "17"];
        cat:hasWell [ a cat:Well;
            cat:hasPlate [ a cat:Plate;
                cat:containerID "1"];
            allores:AFR_0002240 "A1"];
        cat:producesProduct <http://example.org/cat/resource/tFZK0UBWAzCoe3VPYQ1NgSFz1q-ziFOVQmJE_XWRckw>;
        cat:subEquipmentName "GDU-V";
        alloqual:AFQ_0000111 "Liquid";
        allores:AFR_0001606 "addition";
        allores:AFR_0001723 "Chemspeed SWING XL";
        allores:AFR_0002423 "2024-07-25T12:01:35"^^xsd:dateTime;
        allores:AFX_0000622 "2024-07-25T12:01:29"^^xsd:dateTime;
        qudt:quantity [ a cat:Observation;
            cat:errorMargin [ a cat:errorMargin;
                qudt:unit unit:MilliGM;
                qudt:value "0.001"^^xsd:double];
            qudt:unit unit:MilliGM;
            qudt:value "0.024"^^xsd:double].
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}

#[test]
fn test_convert_shake_action() {
    let config = get_test_config("data/tests/synth_shake_action.json");
    let result = json_to_rdf::<SynthBatch>(&config);
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

        <http://example.org/cat/resource/U1-jDX4l3YpJ8VNneXNOyChhCNEV2lBF1387QYXY95A> a cat:Batch;
          purl:identifier "23".

        [] a cat:ShakeAction;
          cat:hasBatch <http://example.org/cat/resource/U1-jDX4l3YpJ8VNneXNOyChhCNEV2lBF1387QYXY95A>;
          cat:hasPlate [ a cat:Plate;
              cat:containerBarcode "1";
              cat:containerID "1"];
          cat:subEquipmentName "Tumble Stirrer";
          cat:temperatureShaker [ a cat:Observation;
              cat:errorMargin [ a cat:errorMargin;
                  qudt:unit unit:DEG_C;
                  qudt:value "2"^^xsd:double];
              qudt:unit unit:DEG_C;
              qudt:value "25"^^xsd:double];
          cat:temperatureTumbleStirrer [ a cat:Observation;
              cat:errorMargin [ a cat:errorMargin;
                  qudt:unit unit:DEG_C;
                  qudt:value "1"^^xsd:double];
              qudt:unit unit:DEG_C;
              qudt:value "25"^^xsd:double];
          alloprop:AFX_0000211 [ a cat:Observation;
              cat:errorMargin [ a cat:errorMargin;
                  qudt:unit unit:REV-PER-MIN;
                  qudt:value "1"^^xsd:double];
              qudt:unit unit:REV-PER-MIN;
              qudt:value "600"^^xsd:double];
          allores:AFR_0001606 "shake";
          allores:AFR_0001723 "Chemspeed SWING XL";
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
fn test_convert_set_vacuum_action() {
    let config = get_test_config("data/tests/synth_set_vacuum_action.json");
    let result = json_to_rdf::<SynthBatch>(&config);
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

        <http://example.org/cat/resource/U1-jDX4l3YpJ8VNneXNOyChhCNEV2lBF1387QYXY95A> a cat:Batch;
          purl:identifier "23".

        [] a cat:SetVacuumAction;
          cat:hasBatch <http://example.org/cat/resource/U1-jDX4l3YpJ8VNneXNOyChhCNEV2lBF1387QYXY95A>;
          cat:hasPlate [ a cat:Plate;
              cat:containerBarcode "1";
              cat:containerID "1"];
          cat:subEquipmentName "vacuum";
          cat:vacuum [ a cat:Observation;
              cat:errorMargin [ a cat:errorMargin;
                  qudt:unit unit:Bar;
                  qudt:value "0.5"^^xsd:double];
              qudt:unit unit:Bar;
              qudt:value "20"^^xsd:double];
          allores:AFR_0001606 "set_vacuum";
          allores:AFR_0001723 "Chemspeed SWING XL";
          allores:AFR_0002423 "2024-07-25T12:03:50"^^xsd:dateTime;
          allores:AFX_0000622 "2024-07-25T12:03:41"^^xsd:dateTime.
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}
