import "./components";
import {Filter} from "@components/types/types";
import {render} from "lit-html";
import {html} from "lit-element";
import {WasmCore} from "@utils/wasm-types";

(window as any).load_wasm((core:WasmCore) => {
    //init wasm
    core.init_app();

    //init event sender

    //kick off the loop where all the magic happens
    const on_tick = (now:DOMHighResTimeStamp) => {
        core.on_tick(now);
        requestAnimationFrame(on_tick);
    }
    requestAnimationFrame(on_tick);

    //handle initial router and changes
    start_router();

    //start initial load
    //send_event(BridgeEvent.InitialLoad);

});

const start_router = () => {
    const get_filter = ():Filter => {
        const page = location.hash.substr(2);
        return page === "active" ? Filter.Active
            : page === "completed" ? Filter.Completed
            : Filter.All
    }

    window.addEventListener('hashchange', () => {
        //send_event([BridgeEvent.FilterChange, get_filter()]);
    });

    //send_event([BridgeEvent.FilterChange, get_filter()]);
}
