use num_traits::FromPrimitive;
use wasm_bindgen::prelude::*;
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
            Ok(None)
            //Err(JsValue::from_str("TODO: remove todo!"))
        },
        BridgeEvent::UpdateTodo => {
            Ok(None)
            //Err(JsValue::from_str("TODO: update todo!"))
        }
        _ => unimplemented!()
    }
}

impl TryFrom<u32> for BridgeEvent {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or_else(|| format!("BridgeEvent: {} is outside of range!", value))
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