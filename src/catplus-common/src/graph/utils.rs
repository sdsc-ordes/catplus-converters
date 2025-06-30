use crate::graph::namespaces::cat_resource;
use base64::{engine::general_purpose, Engine as _};
use sha2::{Digest, Sha256};
use sophia_api::{
    prelude::*,
    term::{bnode_id::BnodeId, SimpleTerm},
};
use uuid::Uuid;

pub fn generate_bnode_term() -> SimpleTerm<'static> {
    let identifier = Uuid::new_v4().to_string();
    let bnode = BnodeId::new_unchecked(identifier);

    bnode.try_into_term().expect("Failed to convert BnodeId to SimpleTerm")
}

/// Hashes an arbitrary identifier string into a URL-safe base64-encoded string.
pub(crate) fn hash_identifier(identifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(identifier.as_bytes());
    let result = hasher.finalize();
    general_purpose::URL_SAFE_NO_PAD.encode(result)
}

pub fn generate_resource_identifier_uri(resource_id: String) -> SimpleTerm<'static> {
    let mut uri = cat_resource::ns.clone().as_str().to_owned();
    uri.push_str(&hash_identifier(&resource_id));
    IriRef::new_unchecked(uri).try_into_term().expect("Failed to convert to SimpleTerm")
}
