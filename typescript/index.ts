import {register_web_components} from "./components";
import {register_event_sender} from "@events/events";
import {render} from "lit-html";
import {html} from "lit-element";
import {WasmCore} from "@utils/wasm-types";
import "./index.css";
register_web_components();

(window as any).load_wasm((core:WasmCore) => {
    core.init_logs();
    const event_queue_ptr = core.init_event_queue();
    const world_ptr = core.init_world();
    register_event_sender(event_queue_ptr) (core.send_event_to_rust);

    const on_tick = (now:DOMHighResTimeStamp) => {
        core.on_tick(world_ptr, event_queue_ptr, now);
        requestAnimationFrame(on_tick);
    }
    requestAnimationFrame(on_tick);
});