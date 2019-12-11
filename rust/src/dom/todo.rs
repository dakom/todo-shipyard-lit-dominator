use wasm_bindgen::prelude::*;
use super::common::get_document;

pub fn replace_items(items:&Vec<&str>) -> Result<(), JsValue> {
    let main_element = main_element()?;
    //let element = page_shadow_root.get_element_by_id(ITEMS_ID).ok_or(JsValue::from_str("couldn't get items container!!"))?;

    let items = serde_wasm_bindgen::to_value(&items)?;
    js_sys::Reflect::set(&main_element, &JsValue::from_str("items"), &items)?;

    Ok(())
}

pub fn main_element() -> Result<web_sys::Element, JsValue> {
    app_root()?
        .get_element_by_id("main").ok_or(JsValue::from_str("couldn't get main element!!"))
}


pub fn app_root() -> Result<web_sys::ShadowRoot, JsValue> {
    let document = get_document()?;
    document.get_element_by_id("app").ok_or(JsValue::from_str("couldn't get app element!!"))?
        .shadow_root().ok_or(JsValue::from_str("couldn't get app shadow root!"))
}