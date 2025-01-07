
use zarrs::group::Group;
use zarrs::array::Array;

use zarrs::storage::ReadableWritableListableStorage;

use serde_json::{json, Value, Map};

fn add_metadata<'all_metadata>(all_metadata: &'all_metadata mut Map<String, Value>, name: &str, metadata: Map<String, Value>)->Result<(), Box<dyn std::error::Error>>{
    all_metadata.insert(name.to_string(), json!(metadata.clone()));
    Ok(())
}

fn iterate_groups<'all_metadata>(store: ReadableWritableListableStorage, group_path: &str, all_metadata: &'all_metadata mut Map<String, Value>)->Result<(), Box<dyn std::error::Error>>{
    let group = Group::open(store.clone(), group_path)?;
    let children_paths = group.child_group_paths(true)?;
    for child_path in children_paths{
        let child_group = Group::open(store.clone(), &child_path.to_string())?;
        println!("Group path: {}", &child_path.to_string());
        let group_attrs = child_group.attributes().clone();
        add_metadata(all_metadata, &child_path.to_string(), group_attrs)?;
        iterate_groups(store.clone(), &child_path.to_string(), all_metadata)?;
        iterate_arrays(store.clone(), &child_path.to_string(), all_metadata)?;
    }
    Ok(())
}

fn iterate_arrays<'all_metadata>(store: ReadableWritableListableStorage, group_path: &str, all_metadata: &'all_metadata mut Map<String, Value>)->Result<(), Box<dyn std::error::Error>>{
    let group = Group::open(store.clone(), group_path)?;
    let children_paths = group.child_array_paths(true)?;
    for child_path in children_paths{
        let child_group = Array::open(store.clone(), &child_path.to_string())?;
        println!("Array path: {}", &child_path.to_string());
        let group_attrs = child_group.attributes().clone();
        add_metadata(all_metadata, &child_path.to_string(), group_attrs)?;
    }
    Ok(())
}

pub fn collect_metadata(store: &mut ReadableWritableListableStorage) -> Result<Value, Box<dyn std::error::Error>> {
    // Iterate over the entire store and collect metadata for all groups and arrays
    let root_path = "/";
    let root_group = Group::open(store.clone(), root_path)?;
    let mut all_metadata = serde_json::Map::new();

    // Collect metadata for the group itself
    let group_attrs: serde_json::Map<std::string::String, Value> = root_group.attributes().clone();
    add_metadata(&mut all_metadata, "root", group_attrs)?;
    println!("Done: {}", "Root group");
    iterate_groups(store.clone(), root_path, &mut all_metadata)?;
    println!("Done: {}", "Group iteration");
    iterate_arrays(store.clone(), root_path, &mut all_metadata)?;
    println!("Done: {}", "Array iteration");

    Ok(json!(all_metadata))
}