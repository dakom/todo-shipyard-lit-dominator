import {LitElement, css, customElement, property} from "lit-element";
import {nothing, html} from "lit-html";
import {repeat} from 'lit-html/directives/repeat';
import {common_css} from "@styles/common";
import {send_event, BridgeEvent} from "@events/events";
import {Item} from "@components/types/types";

@customElement("todo-list")
class List extends LitElement {
    static get styles() { return styles() }

    @property( { type : Array }  ) items = [] as Array<Item>;

    render() {
        const {items} = this;

        return html`
            <ul class="todo-list" id="list">
                ${repeat(
                    items, 
                    item => item,
                    item => html`<todo-item .label=${item} />`
                )}
            </ul>
        `;
    }
}

function styles() {
    return css`
        .todo-list {
            margin: 0;
            padding: 0;
            list-style: none;
        }
    `;
}
export default () => {}