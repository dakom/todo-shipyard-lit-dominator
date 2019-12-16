import { BridgeEvent, send_event } from "@events/events";
import common_css from "@styles/common.css";
import input_css from "@styles/input.css";
import { customElement, html, LitElement, css} from "lit-element";
const ENTER_KEY = 13;

@customElement("todo-input")
class Input extends LitElement {
    static styles = [common_css, input_css];

    render() {
        return html`
            <input id="input-text" class="new-todo" @keydown=${check_keypress} placeholder="What needs to be done?" autofocus />
        `
    }
}

const check_keypress = (evt:KeyboardEvent) => {
    if(evt.keyCode === ENTER_KEY) {
        const input = evt.target as HTMLInputElement;
        const value = input.value.trim();
        if(value !== "") {
            send_event([BridgeEvent.AddTodo, value]);
        }
        input.value = "";
    }
}