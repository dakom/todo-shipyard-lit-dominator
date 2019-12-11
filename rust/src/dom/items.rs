use wasm_bindgen::prelude::*;
use super::page;

const ITEMS_ID:&'static str = "items";

pub fn replace_items(items:&Vec<&str>) -> Result<(), JsValue> {
    let page_shadow_root = page::get_shadow_root()?;
    let element = page_shadow_root.get_element_by_id(ITEMS_ID).ok_or(JsValue::from_str("couldn't get items container!!"))?;

    let items = serde_wasm_bindgen::to_value(&items)?;
    js_sys::Reflect::set(&element, &JsValue::from_str("items"), &items)?;

    Ok(())
}