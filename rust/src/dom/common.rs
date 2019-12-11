use wasm_bindgen::prelude::*;
use web_sys::{window, Window, Document, HtmlElement, Performance};

pub fn get_window() -> Result<Window, JsValue> {
    window().ok_or(JsValue::from_str("could not get window!"))
}

pub fn get_document() -> Result<Document, JsValue> {
    let window = get_window()?;
    window.document().ok_or(JsValue::from_str("could not get document!"))
}

pub fn get_body() -> Result<HtmlElement, JsValue> {
    let document = get_document()?;
    document.body().ok_or(JsValue::from_str("could not get body!"))
}

pub fn get_performance() -> Result<Performance, JsValue> {
    let window = get_window()?;
    window.performance().ok_or(JsValue::from_str("could not get performance!"))
}

//element getters
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