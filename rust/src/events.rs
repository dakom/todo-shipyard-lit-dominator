use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use wasm_bindgen::prelude::*;
use std::convert::{TryFrom, TryInto};
use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "ts_test")] {
        use strum_macros::{EnumIter, AsRefStr};
        use strum::{IntoEnumIterator};
    }
}

#[cfg_attr(feature = "ts_test", derive(EnumIter, AsRefStr))]
#[derive(FromPrimitive, Copy, Clone, Debug)]
#[repr(u32)]
enum BridgeEvent {
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

#[wasm_bindgen]
pub fn send_event_to_rust(evt_type: u32, evt_data: JsValue) -> Result<(), JsValue> {
    let evt_type:BridgeEvent = 
        evt_type.try_into()
            .map_err(|err| JsValue::from_str(&format!("unsupported bridge event! {}", evt_type)))?;

    match evt_type {
        BridgeEvent::AddTodo => {
            log::info!("TODO: Deserialize evt_data as a string!");
            //TODO - then handle it! push to event queue, add component, etc.
        },
        BridgeEvent::RemoveTodo => {
            log::info!("TODO: remove todo!");
        },
        BridgeEvent::UpdateTodo => {
            log::info!("TODO: remove todo!");
        }
    };

    Ok(())
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