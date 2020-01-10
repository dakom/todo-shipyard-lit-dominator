use futures_signals:: {
    signal::{Mutable, Signal, SignalExt, always},
    signal_vec::{SignalVec, SignalVecExt, MutableVec, MutableSignalVec},
    map_ref
};
use dominator::clone;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::components::*;
use crate::actions;
use crate::signals;
use shipyard::prelude::*;
use web_sys::{window, Storage};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;


#[derive(Serialize, Deserialize)]
pub struct StoredData {
    pub items: Vec<(String, bool)>
}

impl StoredData {
    pub fn new<I: IntoIterator<Item=(String, bool)>>(items: I) -> Self {
        Self {
            items: items.into_iter().map(|tuple| (tuple.0, tuple.1)).collect()
        }
    }
}

const STORAGE_NAME:&'static str = "todos-shipyard-lit";

pub fn load_stored_data() -> Option<(StoredData, String)> {
    let local_storage = get_local_storage().unwrap();

    match local_storage.get(STORAGE_NAME).unwrap() {
        None => None,
        Some(json_str) => {
            let data:StoredData = serde_json::from_str(&json_str).map_err(|_| JsValue::from_str("couldn't deserialize storage")).unwrap();
            Some((data, json_str))
        }
    }
}

pub fn save_stored_data(data:&StoredData) -> Result<(), JsValue> {
    let local_storage = get_local_storage()?;

    let json_str = serde_json::to_string(&data).map_err(|_| JsValue::from_str("couldn't serialize storage"))?;

    local_storage.set(STORAGE_NAME, &json_str)
}


fn get_local_storage() -> Result<Storage, JsValue> {
    window().unwrap()
        .local_storage()?
        .ok_or(JsValue::from_str("could not get local storage!"))
}