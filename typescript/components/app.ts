import {LitElement, html, css, customElement} from "lit-element";
import {todo_app} from "@styles/todo-app-css";
import {todo_base} from "@styles/todo-base-css";
import {send_event, BridgeEvent} from "@events/events";
const ENTER_KEY = 13;

export class App extends LitElement {
    static get styles() {
        return [todo_app, todo_base];
    }

    //elements aren't available until _after_ mounting
    //no need to cache them until requested
    _elements: {input: HTMLInputElement};
    get_elements() {
        //cache the element lookups
        if(!this._elements) {
            this._elements = {
                input: this.shadowRoot.getElementById("input-text") as HTMLInputElement
            };
        }

        return this._elements;
    }

    render() {
        const check_keypress = (evt:KeyboardEvent) => {
            if(evt.keyCode === ENTER_KEY) {
                const {value} = this.get_elements().input;

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
            </footer>
        `
    }
}
