#![allow(warnings)]

mod events;
#[path = "components/components.rs"]
mod components;
mod context;
mod dom;
mod systems;
mod world;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use shipyard::prelude::*;
use context::*;
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
pub fn init_app() -> Result<AppContext, JsValue> {
	setup();

    let world = world::init_world();
    let event_queue = VecDeque::new();
    let app_ctx = AppContext { world, event_queue};

    dom::effects::start_dom();

    Ok(app_ctx)
}


#[wasm_bindgen(js_name = send_event_to_rust)]
pub fn on_event_from_js(app_ctx:&mut AppContext, evt_type: u32, evt_data: JsValue) -> Result<(), JsValue> {
    let evt = events::convert_bridge_event(evt_type, evt_data)?;
    if let Some(evt) = evt {
        app_ctx.event_queue.push_back(evt);
    }
    Ok(())
}

#[wasm_bindgen]
pub fn on_tick(app_ctx: &mut AppContext, _now: f64) -> Result<(), JsValue> {
    events::process_events(&mut app_ctx.world, &mut app_ctx.event_queue)?;
    systems::workloads::run_all_workloads(&mut app_ctx.world);
    Ok(())
}