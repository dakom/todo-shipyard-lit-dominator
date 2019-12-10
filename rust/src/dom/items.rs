use crate::events::Page;
use wasm_bindgen::prelude::*;
use web_sys::{window};
use super::common::*;
use super::page::PAGE_ID;

const ITEMS_ID:&'static str = "items";
const ITEMS_ELEM:&'static str = "items-container";

pub fn rewrite_items(items:&Vec<&str>) -> Result<(), JsValue> {

    let document = get_document()?;
    if let Some(elem) = document.get_element_by_id(ITEMS_ID) {
        elem.remove();
    }

    log::info!("hmmm");
    let page_elem = document.get_element_by_id(PAGE_ID).ok_or(JsValue::from_str("no page added yet!!"))?;

    let elem = document.create_element(ITEMS_ELEM)?;

    page_elem.append_child(&elem)?;

    Ok(())
}