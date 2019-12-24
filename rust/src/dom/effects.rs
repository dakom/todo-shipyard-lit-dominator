use wasm_bindgen::prelude::*;
use super::accessors::*;
use serde::{Serialize};
use crate::components::Filter;
use shipyard::EntityId;

#[derive(Serialize)]
pub struct DomItem <'a> {
    id: String,
    label: &'a str,
    completed: bool
}

impl <'a> From<(EntityId, (&'a str, bool))> for DomItem<'a> {
    fn from(tuple: (EntityId, (&'a str, bool))) -> Self {
        Self {
            id: entity_to_id_string(tuple.0),
            label: (tuple.1).0,
            completed: (tuple.1).1
        }
    }
}

fn entity_to_id_string(entity:EntityId) -> String {
    serde_json::to_string(&entity).unwrap()
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

pub fn set_item(entity:EntityId, label:&str, completed: bool) -> Result<(), JsValue> {
    let id = entity_to_id_string(entity);
    let item_element = item_element_by_id(&id)?;

    js_sys::Reflect::set(&item_element, &JsValue::from_str("completed"), &JsValue::from_bool(completed))?;
    js_sys::Reflect::set(&item_element, &JsValue::from_str("label"), &JsValue::from_str(label))?;
    Ok(())
}


pub fn set_top_toggle(all_completed: bool) -> Result<(), JsValue> {
    let main_element = main_element()?;
    js_sys::Reflect::set(&main_element, &JsValue::from_str("all_completed"), &JsValue::from_bool(all_completed))?;
    Ok(())
}