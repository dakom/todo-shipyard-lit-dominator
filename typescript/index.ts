//this will cause all the components to be registered
//note there is no module name
import "./components";

//to get the typescript without needing to go through the webpack dance
import * as _WasmCore from "../_static/wasm/core/pkg/my_core_bg";
type WasmCore = typeof _WasmCore;

//see index.html
(window as any).load_wasm((wasm:WasmCore) => {
    wasm.init_app();
});