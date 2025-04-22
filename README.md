# Cat+ Zarr Converters

## About

This repository contains all the converters for the different data types in the Cat+ project (Agilent, UV, IR, etc.)
The data types are all in different formats, their data and metadata colluded together. The goal will be to convert the metadata to [an established ontology](https://github.com/sdsc-ordes/catplus-ontology/tree/main), and provide the data in their original files.

## Tools

### converter
The converter parses a json input into an rdf graph and serializes the graph to either turtle or jsonld.
It expects the input to conform to the cat+ ontology and the struct `src/catplus-common/src/models/types.rs`. Example input files are provided in `examples` directory.

#### Usage Single File

The `converter` has four arguments:

- input_type: currently `synth` (see `examples/1-Synth.json`) or `hci` (see `examples/0-HCI.json`)
- inputfile: path to input file (relative to top level of the repo or absolute)
- outputfile: path to output file (relative to top level of the repo or absolute)
- format: rdf output format, currently `turtle` or `jsonld`

The `converter` turns the inputfile into a rdf graph and serializes it to either turtle or jsonld. The serialization is written to the provided outputfile.

Examples:

```
just convert synth examples/1-Synth.json examples/1-Synth.ttl turtle
just convert hci examples/0-HCI.json examples/0-HCI.ttl jsonld
just convert agilent examples/2-Agilent.json examples/2-Agilent.ttl turtle
```

#### Usage MultiFile

The `converter-multifile` has four arguments:

- input_folder: folder with different files (HCI, Synth, Agilent) that need to be parsed into RDF
- output_folder: folder where the RDF files are saved
- format: rdf output format, currently `turtle` or `jsonld`

The `converter-multifile` iterates over all files in a folder and parses the filename to know which converter to run on the file. It skips any unknown files or RDF files (with extension ttl or jsonld). It turns each file (containing HCI, Synth or Agilent) into an rdf graph and serializes it to either turtle or jsonld. The serialization is written to the provided output folder.

Example:

```
just convert-multifile examples/complex_model/Bravo2 examples/complex_model/Bravo2 turtle
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
