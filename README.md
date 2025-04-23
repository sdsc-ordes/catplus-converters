# Cat+ Zarr Converters

## About

This repository contains all the converters for the different data types in the Cat+ project (Agilent, UV, IR, etc.)
The data types are all in different formats, their data and metadata colluded together. The goal will be to convert the metadata to [an established ontology](https://github.com/sdsc-ordes/catplus-ontology/tree/main), and provide the data in their original files.

## Tools

### converter
The converter parses a json input into an rdf graph and serializes the graph to either turtle or jsonld.
It expects the input to conform to the cat+ ontology and the struct `src/catplus-common/src/models/types.rs`. Example input files are provided in `examples` directory.

#### Usage

The `converter` has the following arguments:

- input_path: a file or a folder that need to be parsed into RDF
- output_folder (optional): folder where the RDF file(s) will be saved. Defaults to input folder.
- format: rdf output format, currently `turtle` or `jsonld`

The `converter` can read a file or a folder. The parser is selected based on the filename for each input file (HCI, Synth or Agilent), and they are converted to RDF graphs in json-ld or turtle format. The serialization skips unknown files and writes outputs to the input folder, or the provided output folder if specified.

Example for a file:

```
just convert examples/2-Agilent.json turtle
```

Example for a folder (multifiles):

```
just convert examples/complex_model/Bravo2/ turtle
```

Example for a folder (multifiles) where the output folder is specified:

```
just convert examples/complex_model/Bravo2/ turtle --output-folder=examples/rdf
```

### Architecture

The json input is read with `serde_json`: the transformation into rdf is done by the `src/catplus-common` library.
It uses `sophia_rs`. The mapping is triggered by `src/catplus-common/src/models/types.rs` and makes use of the namespaces defined at `src/catplus-common/src/graph/namespaces`.

### Shacl Validation

The rdf graph confirms to the cat+ ontology: https://github.com/sdsc-ordes/catplus-ontology. Currently rust offeres no Shacl Validation Library, but once such a library exists, it would make sense to add a Shacl Validation.

The Shacl Validation can be done manually here: https://www.itb.ec.europa.eu/shacl/any/upload

## Installation guidelines

The repo is setup with nix.

```
git clone git@github.com:sdsc-ordes/catplus-converters.git
cd catplus-converters
cargo build
```

From here on you can work with a just file:

The rust commands can be started via a justfile:

```
just --list
Available recipes:
    build *args                                 # Build all crates
    default                                     # Default recipe to list all recipes.
    format *args                                # Format all crates
    fmt *args                                   # alias for `format`
    nix-develop *args                           # Enter a Nix development shell.
    run input_type input_file output_file *args # Run the converter.
    test *args                                  # Test all crates
```

### Tests

Run the tests with `just test`: only integration tests have been integrated that ensure that the serialized graph in turtle is isomorphic to an expected turtle serialization of the input data.

### Contribute

The repo is a Poc under heavy development and not yet ready to take contributions.
