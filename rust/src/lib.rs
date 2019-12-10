pub mod events;
pub mod components;
mod dom;
mod systems;

use cfg_if::cfg_if;
use log::{info, Level};
use wasm_bindgen::prelude::*;
use shipyard::*;
use events::*;
use systems::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// enable logging only during debug builds
cfg_if! {
    if #[cfg(all(feature = "console_log", debug_assertions))] {
        fn init_log() {
            use console_log;
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log() {}
    }
}

// enable panic hook only during debug builds
cfg_if! {
    if #[cfg(all(feature = "console_error_panic_hook", debug_assertions))] {
        fn init_panic() {
            console_error_panic_hook::set_once();
        }
    } else {
        fn init_panic() {}
    }
}


#[wasm_bindgen]
pub fn init_logs() {
	init_panic();
    init_log();
}

#[wasm_bindgen]
pub struct WorldContext (World);

#[wasm_bindgen]
pub fn init_world() -> WorldContext {

    let world = World::new::<(
        components::Item,
        components::Order,
        components::Dirty
    )>();

    world.add_workload("Render", (RenderDirtyOrder, RenderDirtyItems, RenderClearDirty));

    WorldContext(world)
}

#[wasm_bindgen]
pub struct EventContext (Vec<Event>);

#[wasm_bindgen]
pub fn init_event_queue() -> EventContext {
    EventContext(Vec::new())
}

#[wasm_bindgen]
pub fn send_event_to_rust(event_ctx:&mut EventContext, evt_type: u32, evt_data: JsValue) -> Result<(), JsValue> {
    let evt = convert_bridge_event(evt_type, evt_data)?;
    if let Some(evt) = evt {
        event_ctx.0.push(evt);
    }
    Ok(())
}

#[wasm_bindgen]
pub fn on_tick(world: &mut WorldContext, event_ctx: &mut EventContext, now: f64) -> Result<(), JsValue> {
    process_event_queue(&mut world.0, &mut event_ctx.0, now)?;
    run_systems_tick(&mut world.0, &mut event_ctx.0, now)?;

    Ok(())
}