import {LitElement, css, customElement, property} from "lit-element";
import {nothing, html} from "lit-html";
import {repeat} from 'lit-html/directives/repeat';
import list_css from "@styles/list.css";
import {send_event, BridgeEvent} from "@events/events";
import {Item} from "@components/types/types";

@customElement("todo-list")
class List extends LitElement {
    static styles = list_css;

    @property( { type : Array }  ) items = [] as Array<Item>;

    render() {
        const {items} = this;

        return html`
            <ul class="todo-list" id="list">
                ${repeat(
                    items, 
                    item => item,
                    item => html`<todo-item .label=${item.label} .completed=${item.completed} />`
                )}
            </ul>
        `;
    }
}