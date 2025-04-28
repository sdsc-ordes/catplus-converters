use converter::convert::{ConverterConfig, RdfFormat};
use std::path::{Path, PathBuf};

pub(crate) fn get_data_path(path: &str) -> PathBuf {
    Path::new("../../").join(path)
}

pub(crate) fn get_test_config(input_path: &str) -> ConverterConfig {
    ConverterConfig {
        input_path: get_data_path(input_path),
        format: RdfFormat::Turtle,
        prefix: Some("http://example.org/test/".to_string()),
        materialize: false,
    }
}
