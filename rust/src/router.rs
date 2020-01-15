use shipyard::prelude::*;
use std::rc::Rc;
use gloo_events::EventListener;
use crate::components::*;
use crate::actions;

pub fn start(world:Rc<World>) {

    let window = web_sys::window().unwrap();

    actions::change_filter(&world, get_filter(&window));        

    let listener = EventListener::new(&window, "hashchange", {
        let window = web_sys::window().unwrap();
        move |_event| {
            actions::change_filter(&world, get_filter(&window));        
        }
    });

    listener.forget();

}

fn get_filter(window:&web_sys::Window) -> FilterType {
    match window.location().hash() {
        Err(_) => FilterType::All,
        Ok(hash) => {
            if hash.len() > 2 {
                let hash = &hash[2..];

                match hash {
                    "active" => FilterType::Active,
                    "completed" => FilterType::Completed,
                    _ => FilterType::All
                }
            } else {
                FilterType::All
            }
        }
    }
}