use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use wasm_bindgen::prelude::*;
use std::convert::{TryFrom, TryInto};
use cfg_if::cfg_if;
use super::rust_events::*;

cfg_if! {
    if #[cfg(feature = "ts_test")] {
        use strum_macros::{EnumIter, AsRefStr};
        use strum::{IntoEnumIterator};
    }
}

//Events as sent from JS (straight enum)
#[cfg_attr(feature = "ts_test", derive(EnumIter, AsRefStr))]
#[derive(FromPrimitive, Copy, Clone, Debug)]
#[repr(u32)]
enum BridgeEvent {
    ChangePage,
    AddTodo,
    UpdateTodo,
    RemoveTodo,
}


impl TryFrom<u32> for BridgeEvent {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or_else(|| format!("BridgeEvent: {} is outside of range!", value))
    }
}

impl TryFrom<u32> for Page {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or_else(|| format!("Page: {} is outside of range!", value))
    }
}

pub fn convert_bridge_event(evt_type:u32, evt_data:JsValue) -> Result<Option<Event>, JsValue> {
    let evt_type:BridgeEvent = 
        evt_type.try_into()
            .map_err(|err| JsValue::from_str(&format!("unsupported bridge event! {}", evt_type)))?;


    match evt_type {
        BridgeEvent::ChangePage => {
            let data:Page = (evt_data.as_f64().ok_or(JsValue::from_str(&"unable to get page as number!"))? as u32).try_into()?;
            Ok(Some(Event::ChangePage(data)))
        },
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
    }
}

cfg_if! {
    if #[cfg(feature = "ts_test")] {
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