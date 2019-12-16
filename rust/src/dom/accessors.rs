use super::common::*;
use wasm_bindgen::prelude::*;

pub fn app_root() -> Result<web_sys::ShadowRoot, JsValue> {
    let document = get_document()?;
    document.get_element_by_id("app").ok_or(JsValue::from_str("couldn't get app element!!"))?
        .shadow_root().ok_or(JsValue::from_str("couldn't get app shadow root!"))
}

pub fn main_element() -> Result<web_sys::Element, JsValue> {
    app_root()?
        .get_element_by_id("main").ok_or(JsValue::from_str("couldn't get main element!!"))
}

pub fn footer_element() -> Result<web_sys::Element, JsValue> {
    app_root()?
        .get_element_by_id("footer").ok_or(JsValue::from_str("couldn't get footer element!!"))
}



pub fn footer_shadow_root() -> Result<web_sys::ShadowRoot, JsValue> {
    footer_element()?.shadow_root().ok_or(JsValue::from_str("couldn't get footer shadow root!"))
}

pub fn main_shadow_root() -> Result<web_sys::ShadowRoot, JsValue> {
    main_element()?.shadow_root().ok_or(JsValue::from_str("couldn't get main shadow root!"))
}

pub fn item_element_by_id(id:&str) -> Result<web_sys::Element, JsValue> {

    let main_shadow_root = main_shadow_root()?;
    let list_element = main_shadow_root.get_element_by_id("list").ok_or(JsValue::from_str("couldn't get list element!!"))?;

    main_element()
    /*
    main_shadow_root()?
        .get_element_by_id("list").ok_or(JsValue::from_str("couldn't get list element!!"))?
        .shadow_root().ok_or(JsValue::from_str("couldn't get list shadow root!"))?
        .get_element_by_id(id).ok_or(JsValue::from_str("couldn't get list shadow root!"))
        */
}