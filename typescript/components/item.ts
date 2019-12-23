import {LitElement, css, customElement, property} from "lit-element";
import {nothing, html} from "lit-html";
import {repeat} from 'lit-html/directives/repeat';
import common_css from "@styles/common.css";
import item_css from "@styles/item.css";
import {send_event, BridgeEvent} from "@events/events";
import {Item} from "@components/types/types";
import { classMap } from "lit-html/directives/class-map";

@customElement("todo-item")
class _Item extends LitElement {
    static styles = [common_css, item_css];

    @property( { type : String }  ) label = "";
    @property( { type : String }  ) item_id = "";
    @property( { type : Boolean }  ) completed = false; 

    render() {
        return html`
            <li class=${classMap({completed: this.completed})} >
                <div class="view">
                    <input class="toggle" type="checkbox" .checked=${this.completed} @change=${evt => on_complete_toggle (this.item_id) (evt.target.checked)}/>
                    <label>${this.label}</label>
                    <button class="destroy" @click=${() => on_destroy(this.item_id)} ></button>
                </div>
            </li>
        `;
    }
}

const on_complete_toggle = (id:string) => (completed:boolean) => {
    send_event([BridgeEvent.SetTodoCompleted, [id, completed]]);
}
const on_destroy = (id:string) => {
    send_event([BridgeEvent.RemoveTodo, id]);
}