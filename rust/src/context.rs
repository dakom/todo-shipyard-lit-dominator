use shipyard::prelude::*;
use crate::events::Event;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct AppContext {

    #[wasm_bindgen(skip)]
    pub world: World,

    #[wasm_bindgen(skip)]
    pub event_queue: Vec<Event>,

    #[wasm_bindgen(skip)]
    pub key_cache: KeyCache
}

pub struct KeyCache {
    pub item_list: Key,
}