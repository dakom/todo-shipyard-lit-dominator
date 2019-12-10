import {LitElement, html, css, customElement} from "lit-element";
import {common_css} from "@styles/common";
import {send_event, BridgeEvent} from "@events/events";
const ENTER_KEY = 13;

export class TopInput extends LitElement {
    static get styles() {
        return common_css;
    }

    render() {
        const check_keypress = (evt:KeyboardEvent) => {
            if(evt.keyCode === ENTER_KEY) {
                const input = evt.target as HTMLInputElement;
                send_event([BridgeEvent.AddTodo, input.value]);
                input.value = "";
            }
        }
        return html`
            <input id="input-text" class="new-todo" @keydown=${evt => check_keypress(evt)} placeholder="What needs to be done?" autofocus />
        `
    }
}
