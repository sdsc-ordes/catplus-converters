use lazy_static::lazy_static;
use sophia::api::ns::Namespace;

lazy_static! {
    pub static ref ns: Namespace<&'static str> = Namespace::new("http://example.org/cat/resource/").unwrap();
}

