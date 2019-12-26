use super::{render, save, dirty, events};

pub fn register_workloads(world:&mut shipyard::World) {
    world.add_workload("ProcessEvents", (events::Add));
    world.add_workload("Render", (render::RenderList, render::RenderCompleted, render::RenderFilter, render::RenderContents, render::RenderTopToggle));
    world.add_workload("Save", save::Save); 
    world.add_workload("PostRender", dirty::ClearDirty);
}

pub fn run_all_workloads(world:&mut shipyard::World) {
    world.run_workload("ProcessEvents");
    world.run_workload("Render");
    world.run_workload("Save");
    world.run_workload("PostRender");
}