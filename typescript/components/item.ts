import {LitElement, css, customElement, property} from "lit-element";
import {nothing, html} from "lit-html";
import {repeat} from 'lit-html/directives/repeat';
import common_css from "@styles/common.css";
import item_css from "@styles/item.css";
import {send_event, BridgeEvent} from "@events/events";
import { classMap } from "lit-html/directives/class-map";
import {KEYS} from "@components/types/types";

@customElement("todo-item")
class _Item extends LitElement {
    static styles = [common_css, item_css];

    @property( { type : String }  ) label = "";
    @property( { type : String }  ) item_id = "";
    @property( { type : Boolean }  ) completed = false; 
    @property( { type : Boolean }  ) editing = false; 

    render() {
        return this.editing
            ? html`<todo-edit-line @stop-editing=${() => this.editing = false} .label=${this.label} .item_id=${this.item_id} />` 
            : html`
                <li class=${classMap({completed: this.completed})} >
                    <div class="view">
                        <input class="toggle" type="checkbox" .checked=${this.completed} @change=${evt => on_complete_toggle (this.item_id) (evt.target.checked)}/>
                        <label @dblclick=${() => this.editing = true}>${this.label}</label>
                        <button class="destroy" @click=${() => on_destroy(this.item_id)} ></button>
                    </div>
                </li>
            `;
    }
}

@customElement("todo-edit-line")
class EditLine extends LitElement {
    static styles = [common_css, item_css];

    @property( { type : String }  ) label = "";
    @property( { type : String }  ) item_id = "";

    stop_editing() {
        this.dispatchEvent(new Event("stop-editing"));
    }

    firstUpdated() {
        this.shadowRoot.getElementById("input").focus();
    }

    render() {
        return html`
                <li class="editing" >
                    <input id="input" type="text" class="edit" .value=${this.label} 
                        @blur=${this.stop_editing}
                        @keydown=${evt => {
                            if(check_edit_keypress (this.item_id) (evt)) {
                                this.stop_editing();
                            }
                        }} 
                    />
                </li>
            ` 
    }
}

const check_edit_keypress = (id:string) => (evt:KeyboardEvent):boolean => {
    const {keyCode} = evt;

    if(keyCode !== KEYS.ENTER && keyCode !== KEYS.ESC) {
        return false;
    } 
    if(evt.keyCode === KEYS.ENTER) {
        const input = evt.target as HTMLInputElement;
        const value = input.value.trim();
        if(value !== "") {
            send_event([BridgeEvent.ChangeTodo, [id, value]]);
        }
        input.value = "";
    }
    return true;

}
const on_complete_toggle = (id:string) => (completed:boolean) => {
    send_event([BridgeEvent.SetCompleted, [id, completed]]);
}

const on_destroy = (id:string) => {
    send_event([BridgeEvent.RemoveTodo, id]);
}