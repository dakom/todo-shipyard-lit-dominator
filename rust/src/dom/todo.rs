use wasm_bindgen::prelude::*;
use super::common::*;

pub fn replace_items(items:JsValue) -> Result<(), JsValue> {
    let main_element = main_element()?;

    js_sys::Reflect::set(&main_element, &JsValue::from_str("items"), &items)?;

    Ok(())
}

pub fn update_status() -> Result<(), JsValue> {
    let footer_element = footer_element()?;
    js_sys::Reflect::set(&footer_element, &JsValue::from_str("count"), &JsValue::from_f64(2.0))?;

    Ok(())
}
