import {LitElement, css, customElement, property} from "lit-element";
import {nothing, html} from "lit-html";
import {repeat} from 'lit-html/directives/repeat';
import common_css from "@styles/common.css";
import item_css from "@styles/item.css";
import { classMap } from "lit-html/directives/class-map";
import {KEYS, Item} from "@components/types/types";
import {RemoveTodo, ToggleTodo, ChangeTodo} from "@events/events";

@customElement("todo-item")
class _Item extends LitElement {
    static styles = [common_css, item_css];

    @property( { type : Object }  ) item = null as Item;
    @property( { type : Boolean }  ) editing = false; 

    render() {
        const {label, id, complete} = this.item;

        const on_remove = () => this.dispatchEvent(new RemoveTodo({id}));
        const on_toggle = (evt:any) => this.dispatchEvent(new ToggleTodo({id, complete: evt.target.checked}));
        const on_change = (label:string) => this.dispatchEvent(new ChangeTodo({id, label}));

        return this.editing
            ? html`<todo-edit-line @stop-editing=${() => this.editing = false} .on_change=${on_change} .label=${label} .item_id=${id} />` 
            : html`
                <li class=${classMap({completed: complete})} >
                    <div class="view">
                        <input class="toggle" type="checkbox" .checked=${complete} @change=${on_toggle.bind(this)}/>
                        <label @dblclick=${() => this.editing = true}>${label}</label>
                        <button class="destroy" @click=${on_remove.bind(this)} ></button>
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
    @property( { type: Function} ) on_change = null as (label:string) => any;

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
                            const {keyCode} = evt;

                            if(keyCode === KEYS.ENTER || keyCode === KEYS.ESC) {
                                if(evt.keyCode === KEYS.ENTER) {
                                    const input = evt.target as HTMLInputElement;
                                    const value = input.value.trim();
                                    if(value !== "") {
                                        this.on_change(value);
                                    }
                                    input.value = "";
                                }

                                this.stop_editing();
                            } 
                        }} 
                    />
                </li>
            ` 
    }
}