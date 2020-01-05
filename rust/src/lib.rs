#![allow(warnings)]

mod events;
#[path = "components/components.rs"]
mod components;
mod dom;
mod systems;
mod world;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use shipyard::prelude::*;
use std::collections::VecDeque;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// enable logging and panic hook only during debug builds
cfg_if! {
    if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook", debug_assertions))] {
        fn setup() {
            wasm_logger::init(wasm_logger::Config::default());
            console_error_panic_hook::set_once();
            log::info!("rust logging enabled!!!");
        }
    } else {
        fn setup() {
            log::info!("rust logging disabled!"); //<-- won't be seen
        }
    }
}

#[wasm_bindgen]
pub fn init_app() {
	setup();

    dominator::append_dom(&dominator::body(), dom::tree::get_dom_tree());
}

#[wasm_bindgen]
pub fn on_tick(_now: f64) -> Result<(), JsValue> {
    systems::workloads::run_all_workloads(&world::WORLD);
    Ok(())
}