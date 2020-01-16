import common_css from "@styles/common.css";
import input_css from "@styles/input.css";
import { customElement, html, LitElement, css} from "lit-element";
import {KEYS} from "@utils/keys";
import { AddTodo } from "@events/events";

@customElement("todo-input")
class Input extends LitElement {
    static styles = [common_css, input_css];

    check_keypress (evt:KeyboardEvent) {
        if(evt.keyCode === KEYS.ENTER) {
            const input = evt.target as HTMLInputElement;
            const label = input.value.trim();
            if(label !== "") {
                this.dispatchEvent(new AddTodo({label}));
            }
            input.value = "";
        }
    }
    render() {
        return html`
            <input id="input-text" class="new-todo" @keydown=${this.check_keypress} placeholder="What needs to be done?" autofocus />
        `
    }
}
