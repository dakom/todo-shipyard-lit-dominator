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