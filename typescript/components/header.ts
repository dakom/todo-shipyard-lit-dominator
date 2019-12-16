import header_css from "@styles/header.css";
import { customElement, LitElement } from "lit-element";
import { html } from "lit-html";
const ENTER_KEY = 13;

@customElement("todo-header")
export class Items extends LitElement {
    static styles = header_css;
    render() {
        return html`
            <header class="header">
                <h1>todos</h1>
                <todo-input id="input"></todo-input>
            </header>
        `;
    }
}