use shipyard::prelude::*;
use std::rc::Rc;
use gloo_timers::callback::Interval;
use crate::systems;

pub fn start(world:Rc<World>) {
    //check every 42ms if we need to save
    let timeout = Interval::new(42, move || {
        world.run_workload(systems::SAVE);
    });

    // Since we don't plan on cancelling the timeout, call `forget`.
    timeout.forget();
}