use wasm_bindgen::prelude::*;
use super::common::*;

pub const PAGE_ID:&'static str = "page";

pub fn start_page() -> Result<(), JsValue> {
    let document = get_document()?;
    let body = get_body()?;
    let elem = document.create_element("main-page")?;
    elem.set_id(PAGE_ID);
    body.append_child(&elem)?;

    Ok(())
}

pub fn get_shadow_root() -> Result<web_sys::ShadowRoot, JsValue> {
    let document = get_document()?;
    let page_element = document.get_element_by_id(PAGE_ID).ok_or(JsValue::from_str("couldn't get page container!!"))?;
    page_element.shadow_root().ok_or(JsValue::from_str("couldn't get page shadow root!!"))
}