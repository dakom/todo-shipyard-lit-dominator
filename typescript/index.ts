import "./components";
import {Filter} from "@components/types/types";
import {render} from "lit-html";
import {html} from "lit-element";
import {WasmCore} from "@utils/wasm-types";

(window as any).load_wasm((wasm:WasmCore) => {
    //init wasm
    const app_ctx = wasm.init_app();

    //TODO: move to Rust in order to run systems (check for save every second or so)
    setInterval(() => wasm.on_tick(app_ctx), 1000);
});