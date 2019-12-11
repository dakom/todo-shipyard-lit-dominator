use wasm_bindgen::prelude::*;
use serde::{Serialize};

#[derive(Serialize)]
pub struct Item {
    pub label: String,
    pub completed: bool,
}

impl Item {
    pub fn new(label:&str) -> Self {
        Self {
            label: label.to_string(),
            completed: false
        }
    }
}

pub struct ItemList { }
