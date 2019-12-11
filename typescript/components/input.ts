import { BridgeEvent, send_event } from "@events/events";
import { common_css } from "@styles/common";
import { customElement, html, LitElement, css} from "lit-element";
const ENTER_KEY = 13;

@customElement("todo-input")
class Input extends LitElement {
    static get styles() { return styles() }

    render() {
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
        return html`
            <input id="input-text" class="new-todo" @keydown=${evt => check_keypress(evt)} placeholder="What needs to be done?" autofocus />
        `
    }
}

function styles() {
        return [common_css, css`

            input::-webkit-input-placeholder {
                font-style: italic;
                font-weight: 300;
                color: rgba(0, 0, 0, 0.4);
            }

            input::-moz-placeholder {
                font-style: italic;
                font-weight: 300;
                color: rgba(0, 0, 0, 0.4);
            }

            input::input-placeholder {
                font-style: italic;
                font-weight: 300;
                color: rgba(0, 0, 0, 0.4);
            }

            input.new-todo {
                padding: 16px 16px 16px 60px;
                border: none;
                background: rgba(0, 0, 0, 0.003);
                box-shadow: inset 0 -2px 1px rgba(0,0,0,0.03);
            }
        `];
}