use crate::events::Page;
use wasm_bindgen::prelude::*;
use web_sys::{window};
use super::common::*;
use super::page::PAGE_ID;

const ITEMS_ID:&'static str = "items";
const ITEMS_ELEM:&'static str = "items-container";
const ITEMS_SLOT_NAME:&'static str = "items";
const ITEM_ELEM:&'static str = "item-container";

pub fn rewrite_items(items:&Vec<&str>) -> Result<(), JsValue> {

    let document = get_document()?;

    let page_elem = document.get_element_by_id(PAGE_ID).ok_or(JsValue::from_str("no page added yet!!"))?;

    let page_shadow = page_elem.shadow_root().ok_or(JsValue::from_str("could not get shadow root"))?;

    if let Some(elem) = document.get_element_by_id(ITEMS_ID) {
        log::info!("removed element!");
        elem.remove();
    } else {
        log::info!("no element!");
    }


    let items_container = document.create_element(ITEMS_ELEM)?;
    items_container.set_id(ITEMS_ID);
    items_container.set_attribute("slot", ITEMS_SLOT_NAME)?;

    for item in items.iter() {
        let item_container = document.create_element(ITEM_ELEM)?;
        item_container.set_attribute("label", item)?;
        items_container.append_child(&item_container)?;
    }
    page_elem.append_child(&items_container)?;

    Ok(())
}