use wasm_bindgen::prelude::*;
use super::accessors::*;
use serde::{Serialize, Deserialize};
use crate::components::Filter;
use shipyard::EntityId;

#[derive(Serialize, Deserialize)]
pub struct StoredData {
    pub items: Vec<(String, bool)>
}

impl StoredData {
    pub fn new<'a, I: IntoIterator<Item=(&'a str, bool)>>(items: I) -> Self {
        Self {
            items: items.into_iter().map(|tuple| (tuple.0.to_string(), tuple.1)).collect()
        }
    }
}

const STORAGE_NAME:&'static str = "todos-shipyard-lit";

pub fn load_stored_data() -> Result<Option<(StoredData, String)>, JsValue> {
    let local_storage = get_local_storage()?;

    match local_storage.get(STORAGE_NAME)? {
        None => Ok(None),
        Some(json_str) => {
            let data:StoredData = serde_json::from_str(&json_str).map_err(|_| JsValue::from_str("couldn't deserialize storage"))?;
            Ok(Some((data, json_str)))
        }
    }
}

pub fn save_stored_data(data:&StoredData) -> Result<(), JsValue> {
    let local_storage = get_local_storage()?;

    let json_str = serde_json::to_string(&data).map_err(|_| JsValue::from_str("couldn't serialize storage"))?;

    local_storage.set(STORAGE_NAME, &json_str)
}