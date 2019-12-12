use super::common::*;
use wasm_bindgen::prelude::*;

pub fn main_element() -> Result<web_sys::Element, JsValue> {
    app_root()?
        .get_element_by_id("main").ok_or(JsValue::from_str("couldn't get main element!!"))
}

pub fn footer_element() -> Result<web_sys::Element, JsValue> {
    app_root()?
        .get_element_by_id("footer").ok_or(JsValue::from_str("couldn't get footer element!!"))
}

pub fn app_root() -> Result<web_sys::ShadowRoot, JsValue> {
    let document = get_document()?;
    document.get_element_by_id("app").ok_or(JsValue::from_str("couldn't get app element!!"))?
        .shadow_root().ok_or(JsValue::from_str("couldn't get app shadow root!"))
}


pub fn filter_element() -> Result<web_sys::Element, JsValue> {
    app_root()?
        .get_element_by_id("main").ok_or(JsValue::from_str("couldn't get main element!!"))
}