import {LitElement, html, css, customElement} from "lit-element";
import {common_css} from "@styles/common";
import {send_event, BridgeEvent} from "@events/events";
const ENTER_KEY = 13;

export class MainPage extends LitElement {
    static get styles() {
        return common_css;
    }

    render() {
        const check_keypress = (evt:KeyboardEvent) => {
            if(evt.keyCode === ENTER_KEY) {
                const {value} = evt.target as HTMLInputElement;
                send_event([BridgeEvent.AddTodo, value]);
            }
        }
        return html`
            <section class="todoapp">
                <header class="header">
                    <h1>todos</h1>
                    <input id="input-text" class="new-todo" @keydown=${evt => check_keypress(evt)} placeholder="What needs to be done?" autofocus />
                </header>
            </section>
            <footer class="info">
                <p>Double-click to edit a todo</p>
                <p><a href="https://github.com/dakom/todo-shipyard-lit"><u>Repo on Github</u></a></p>
            </footer>
        `
    }
}
