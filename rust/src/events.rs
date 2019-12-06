use wasm_bindgen::prelude::*;
use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
enum BridgeEvent {
    AddTodo,
}

impl TryFrom<u32> for BridgeEvent {
    type Error = JsValue;

    fn try_from(value:u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AddTodo),
            _ => Err(JsValue::from_str(&format!("{} is not a supported BridgeEvent value!", value)))
        }
    }
}

#[wasm_bindgen]
pub fn send_event_to_rust(evt_type: u32, evt_data: JsValue) -> Result<(), JsValue> {
    let evt_type:BridgeEvent = evt_type.try_into()?;

    match evt_type {
        BridgeEvent::AddTodo => {
            log::info!("TODO: Deserialize evt_data as a string!");
            //TODO - then handle it! push to event queue, add component, etc.
        }
    };

    Ok(())
}