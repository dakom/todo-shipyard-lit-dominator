use num_traits::FromPrimitive;
use wasm_bindgen::prelude::*;
use crate::components::Filter;
use std::convert::{TryFrom, TryInto};
use cfg_if::cfg_if;
use super::events::*;

//convert bridge events (and their data) to rust events
pub fn convert_bridge_event(evt_type:u32, evt_data:JsValue) -> Result<Option<Event>, JsValue> {
    let evt_type:BridgeEvent = 
        evt_type.try_into()
            .map_err(|err:String| JsValue::from_str(&err))?;

    match evt_type {
        BridgeEvent::AddTodo => {
            let data:String = serde_wasm_bindgen::from_value(evt_data)?;
            Ok(Some(Event::AddTodo(data)))
        },
        BridgeEvent::RemoveTodo => {
            //TODO - use proper key id. See https://github.com/leudz/shipyard/issues/23
            let index:String = evt_data.as_string().ok_or(JsValue::from_str("invalid id"))?;
            let index:usize = index.parse().map_err(|_| JsValue::from_str("invalid id"))?;
            Ok(Some(Event::RemoveTodo(index as usize)))
        },
        BridgeEvent::UpdateTodo => {
            Ok(None)
            //Err(JsValue::from_str("TODO: update todo!"))
        },
        BridgeEvent::FilterChange => {
            let filter:f64 = evt_data.as_f64().ok_or(JsValue::from_str("invalid filter"))?;
            let filter:Filter = (filter as u32).try_into()?;
            Ok(Some(Event::FilterChange(filter)))
        },
        _ => unimplemented!()
    }
}


cfg_if! {
    if #[cfg(feature = "ts_test")] {
        use strum::{IntoEnumIterator};

        #[wasm_bindgen]
        pub fn get_bridge_event_pairs() -> Vec<JsValue> {
            BridgeEvent::iter()
                .map(|evt| {
                    let index = evt as u32;
                    let name = evt.as_ref();
                    serde_wasm_bindgen::to_value(&(index, name)).unwrap()
                })
                .collect()
        }
    }
}