mod events;
mod components;
mod context;
mod dom;
mod systems;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use shipyard::*;
use events::{convert_events::*, process_events::*};
use components::*;
use systems::*;
use context::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// enable logging and panic hook only during debug builds
cfg_if! {
    if #[cfg(all(feature = "console_log", feature = "console_error_panic_hook", debug_assertions))] {
        fn setup() {
            use console_log;
            console_log::init_with_level(log::Level::Trace).expect("error initializing log");
            console_error_panic_hook::set_once();
            log::info!("rust logging enabled!");
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

    let (world, key_cache) = init_world();
    let event_queue:Vec<events::Event> = Vec::new();

    Ok(AppContext {
        world,
        event_queue,
        key_cache
    })
}

fn init_world() -> (World, KeyCache) {
    let world = World::new::<(
        components::Item,
        components::ItemList,
        components::Dirty
    )>();

    world.add_workload("Render", (render::ItemList, render::ItemsUpdate, render::ClearDirty));

    let mut item_list_key:Option<Key> = None;
    world.run::<(EntitiesMut, &mut ItemList), _>(|(mut entities, mut item_lists)| {
        item_list_key = Some(entities.add_entity(&mut item_lists, ItemList {}));
    });

    (world, KeyCache { item_list: item_list_key.unwrap()})
}

#[wasm_bindgen(js_name = send_event_to_rust)]
pub fn on_event_from_js(app_ctx:&mut AppContext, evt_type: u32, evt_data: JsValue) -> Result<(), JsValue> {
    let event_queue = &mut app_ctx.event_queue;
    let evt = convert_bridge_event(evt_type, evt_data)?;
    if let Some(evt) = evt {
        event_queue.push(evt);
    }
    Ok(())
}

#[wasm_bindgen]
pub fn on_tick(app_ctx: &mut AppContext, now: f64) -> Result<(), JsValue> {
    process_events(app_ctx, now)?;
    app_ctx.world.run_workload("Render");

    Ok(())
}