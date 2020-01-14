#![allow(warnings)]

mod actions;
mod components;
mod dom;
mod events;
mod signals;
mod storage;
mod systems;
mod world;
mod router;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use shipyard::prelude::*;
use std::collections::VecDeque;
use std::rc::Rc;

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
pub struct AppContext {
    #[wasm_bindgen(skip)]
    pub world: Rc<World>,
}

#[wasm_bindgen]
pub fn init_app() -> AppContext {
	setup();

    let world = Rc::new(world::init_world());
    systems::register_workloads(&world);
    dominator::append_dom(&dominator::body(), dom::render(world.clone()));
    actions::spawn_save_listener(world.clone());
    actions::load(world.clone());
    router::start(world.clone());
    AppContext { world }
}

#[wasm_bindgen]
pub fn on_tick(app_ctx:&mut AppContext) {
    app_ctx.world.run_workload(systems::SAVE);
}