use std::path::{Path, PathBuf};
use converter::convert::ConverterConfig;
use converter::convert::RdfFormat;

pub(crate) fn get_data_path(path: &str) -> PathBuf {
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    project_root.join(path)
}

pub(crate) fn get_test_config(input_path: &str) -> ConverterConfig {
    ConverterConfig {
        input_path: get_data_path(input_path),
        format: RdfFormat::Turtle,
        prefix: None,
        materialize: false,
    }
}
