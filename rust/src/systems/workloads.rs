use super::{render, save, clear};

pub fn register_workloads(world:&shipyard::World) {
    world.add_workload("Render", (render::RenderList, render::RenderCompleted, render::RenderFilter, render::RenderContents, render::RenderTopToggle));
    world.add_workload("Save", save::Save); 
    world.add_workload("ClearUpdated", clear::ClearUpdated); 

}

pub fn run_all_workloads(world:&shipyard::World) {
    world.run_workload("Render");
    world.run_workload("Save");
    world.run_workload("ClearUpdated");
}