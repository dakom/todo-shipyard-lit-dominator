import {LitElement, css, customElement, property} from "lit-element";
import {nothing, html} from "lit-html";
import {repeat} from 'lit-html/directives/repeat';
import list_css from "@styles/list.css";
import {Item} from "@components/types/types";

@customElement("todo-list")
class List extends LitElement {
    static styles = list_css;

    @property( { type : Array }  ) items = [] as Array<Item>;

    render() {
        const {items} = this;

        return html`
            <ul class="todo-list">
                ${repeat(
                    items, 
                    item => item.id,
                    item => html`<todo-item id=${item.id} .item_id=${item.id} .label=${item.label} .completed=${item.completed} />`
                )}
            </ul>
        `;
    }
}