import {register_web_components} from "./components";
import {register_event_sender} from "@events/events";
import {render} from "lit-html";
import {html} from "lit-element";
import {WasmCore} from "@utils/wasm-types";
import "./index.css";

register_web_components();

(window as any).load_wasm((core:WasmCore) => {
    const rust_app_ctx_ptr = core.init_app();
    register_event_sender(rust_app_ctx_ptr) (core.send_event_to_rust);

    const on_tick = (now:DOMHighResTimeStamp) => {
        core.on_tick(rust_app_ctx_ptr, now);
        requestAnimationFrame(on_tick);
    }
    requestAnimationFrame(on_tick);
});