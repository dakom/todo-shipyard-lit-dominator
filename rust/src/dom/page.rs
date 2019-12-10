use crate::events::Page;
use wasm_bindgen::prelude::*;
use web_sys::{window};
use super::common::*;

pub const PAGE_ID:&'static str = "page";
const MAIN_PAGE_ELEM:&'static str = "main-page";

pub fn change_page(page:Page) -> Result<(), JsValue> {
    let document = get_document()?;

    if let Some(elem) = document.get_element_by_id(PAGE_ID) {
        elem.remove();
    }
    let body = get_body()?;
    let elem = document.create_element(MAIN_PAGE_ELEM)?;
    elem.set_id(PAGE_ID);
    body.append_child(&elem)?;

    Ok(())
}