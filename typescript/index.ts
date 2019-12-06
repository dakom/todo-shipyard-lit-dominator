import {register_web_components} from "@components/components";
import {WasmCore} from "@utils/wasm-types";
import "./index.css";
import "todomvc-app-css/index.css";
import "todomvc-common/base.css";

register_web_components();

(window as any).load_wasm((core:WasmCore) => {
    core.init_app();
    document.body.appendChild(document.createElement("my-app"));
});