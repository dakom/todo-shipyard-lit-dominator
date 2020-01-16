import {LitElement, css, customElement, property} from "lit-element";
import {html} from "lit-html";
import list_css from "@styles/list.css";

@customElement("todo-list")
class List extends LitElement {
    static styles = list_css;

    render() {
        return html`
            <ul class="todo-list">
                <slot></slot>
            </ul>
        `;
    }
}