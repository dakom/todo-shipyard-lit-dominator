use wasm_bindgen::prelude::*;
use super::accessors::*;
use serde::{Serialize};
use crate::components::Filter;

#[derive(Serialize)]
pub struct DomItem <'a> {
    label: &'a str,
    completed: bool
}

impl <'a> From<(&'a str, bool)> for DomItem<'a> {
    fn from(tuple: (&'a str, bool)) -> Self {
        Self {
            label: tuple.0,
            completed: tuple.1
        }
    }
}

pub fn set_items(items:Vec<DomItem>) -> Result<(), JsValue> {
    let main_element = main_element()?;
    let items = serde_wasm_bindgen::to_value(&items).unwrap();
    js_sys::Reflect::set(&main_element, &JsValue::from_str("items"), &items)?;

    Ok(())
}

pub fn set_count(total:usize, remaining:usize) -> Result<(), JsValue> {
    let footer_element = footer_element()?;
    js_sys::Reflect::set(&footer_element, &JsValue::from_str("count"), &JsValue::from_f64(remaining as f64))?;

    Ok(())
}

pub fn set_filter(filter:Filter) -> Result<(), JsValue> {
    let filter_element = filter_element()?;

    js_sys::Reflect::set(&filter_element, &JsValue::from_str("filter"), &JsValue::from_str(&filter.to_string()))?;
    Ok(())
}