import "./components"
import {register_event_sender} from "@events/events";
import {render} from "lit-html";
import {html} from "lit-element";
import {WasmCore} from "@utils/wasm-types";
import "./index.css";


(window as any).load_wasm((core:WasmCore) => {
    //init wasm
    const rust_app_ctx_ptr = core.init_app();

    //init event sender
    register_event_sender(rust_app_ctx_ptr) (core.send_event_to_rust);

    //mount the app 
    const element = document.createElement("todo-app");
    element.id = "app";
    document.body.appendChild(element);

    //kick off the loop where all the magic happens
    const on_tick = (now:DOMHighResTimeStamp) => {
        core.on_tick(rust_app_ctx_ptr, now);
        requestAnimationFrame(on_tick);
    }
    requestAnimationFrame(on_tick);

});