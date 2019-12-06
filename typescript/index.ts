import {register_web_components} from "./components";
import {register_event_sender} from "@events/events";
import {render} from "lit-html";
import {html} from "lit-element";
import {WasmCore} from "@utils/wasm-types";
import "./index.css";

const root_element = document.getElementById("root");

register_web_components();

(window as any).load_wasm((core:WasmCore) => {
    core.init_app();
    register_event_sender(core.send_event_to_rust);
    const on_tick = () => {
        render_everything();
        requestAnimationFrame(on_tick);
    }
    on_tick();

});

function render_everything() {
    render(html`<my-app></my-app>`, root_element);
}