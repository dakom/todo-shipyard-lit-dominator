use shipyard::prelude::*;
use crate::events::Event;
use wasm_bindgen::prelude::*;
use std::collections::VecDeque;

#[wasm_bindgen]
pub struct AppContext {
    #[wasm_bindgen(skip)]
    pub world: World,
    #[wasm_bindgen(skip)]
    pub event_queue: VecDeque<Event>,
}