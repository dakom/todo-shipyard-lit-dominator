mod render;
mod dirty;

pub fn register_workloads(world:&mut shipyard::World) {
    world.add_workload("Render", (render::RenderList, render::RenderCompleted, render::RenderFilter, render::RenderContents));
    world.add_workload("PostRender", dirty::ClearDirty);
}

pub fn run_all_workloads(world:&mut shipyard::World) {
    world.run_workload("Render");
    world.run_workload("PostRender");
}