use wasm_bindgen::prelude::*;
use super::accessors::*;
use serde::{Serialize};
use crate::components::Filter;

#[derive(Serialize)]
pub struct DomItem <'a> {
    id: String,
    label: &'a str,
    completed: bool
}

impl <'a> From<(String, &'a str, bool)> for DomItem<'a> {
    fn from(tuple: (String, &'a str, bool)) -> Self {
        Self {
            id: tuple.0,
            label: tuple.1,
            completed: tuple.2
        }
    }
}

pub fn set_items(items:Vec<DomItem>) -> Result<(), JsValue> {
    let main_element = main_element()?;
    let items = serde_wasm_bindgen::to_value(&items).unwrap();
    js_sys::Reflect::set(&main_element, &JsValue::from_str("items"), &items)?;

    Ok(())
}

pub fn set_count(total:usize, completed:usize,  remaining:usize) -> Result<(), JsValue> {
    let footer_element = footer_element()?;
    js_sys::Reflect::set(&footer_element, &JsValue::from_str("total"), &JsValue::from_f64(total as f64))?;
    js_sys::Reflect::set(&footer_element, &JsValue::from_str("remaining"), &JsValue::from_f64(remaining as f64))?;
    js_sys::Reflect::set(&footer_element, &JsValue::from_str("completed"), &JsValue::from_f64(completed as f64))?;

    Ok(())
}

pub fn set_filter(filter:Filter) -> Result<(), JsValue> {
    let footer_element = footer_element()?;

    js_sys::Reflect::set(&footer_element, &JsValue::from_str("filter"), &JsValue::from_f64(filter as u32 as f64))?;
    Ok(())
}