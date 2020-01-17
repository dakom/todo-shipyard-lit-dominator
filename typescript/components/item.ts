import {LitElement, css, customElement, property} from "lit-element";
import {nothing, html} from "lit-html";
import {repeat} from 'lit-html/directives/repeat';
import common_css from "@styles/common.css";
import item_css from "@styles/item.css";
import { classMap } from "lit-html/directives/class-map";
import {Item, DropSide} from "@events/types/types";
import {KEYS} from "@utils/keys";
import {RemoveTodo, ToggleTodo, ChangeTodo, Reposition} from "@events/events";


@customElement("todo-item")
class _Item extends LitElement {
    static styles = [common_css, item_css];

    @property( { type : Object }  ) item = null as Item;
    @property( { type : Boolean }  ) editing = false; 
    @property( { type: Boolean} ) draggable = false;
    @property( { type: Boolean} ) dragging = false;
    @property( { type: Number } ) dropside = DropSide.None;

    render() {
        const {label, id, complete} = this.item;

        const on_remove = () => this.dispatchEvent(new RemoveTodo({id}));
        const on_toggle = (evt:any) => this.dispatchEvent(new ToggleTodo({id, complete: evt.target.checked}));
        const on_change = (label:string) => this.dispatchEvent(new ChangeTodo({id, label}));
       
        //drag handlers
        const on_dragicon_hover = () => this.draggable = true;
        const on_dragstart = (evt:DragEvent) => {
            requestAnimationFrame(() => this.dragging = true);
            evt.dataTransfer.setData("text/plain", JSON.stringify(id));
        }
        const on_dragend = (evt:DragEvent) => {
            this.dragging = false;
        }

        const on_drop = (evt:DragEvent) => {
            const drag_id = JSON.parse(evt.dataTransfer.getData("text/plain"));

            if(this.dropside !== DropSide.None) {
                this.dispatchEvent(new Reposition({
                    src: drag_id,
                    dest: id,
                    side: this.dropside
                }));
            }

            this.dropside = DropSide.None;
            evt.preventDefault();
        }
        const set_dropside = (evt:DragEvent) => {
            if(!this.dragging) {
                //based on CSS, which is using hard pixel values anyway
                this.dropside = (evt.offsetY < 25) ? DropSide.Before : DropSide.After;
                evt.preventDefault();
            }
        }
        const on_dragenter = set_dropside; 
        const on_dragover = set_dropside; 
        const on_dragleave = (evt:DragEvent) => this.dropside = DropSide.None;


        return this.editing
            ? html`<todo-edit-line @stop-editing=${() => this.editing = false} .on_change=${on_change} .label=${label} .item_id=${id} />` 
            : html`
                <li class=${classMap({completed: complete, hidden: this.dragging})} 
                    draggable=${this.draggable} 
                    @dragend=${on_dragend.bind(this)} 
                    @dragover=${on_dragover.bind(this)}
                    @dragenter=${on_dragenter.bind(this)}
                    @dragleave=${on_dragleave.bind(this)}
                    @dragstart=${on_dragstart.bind(this)}
                    @drop=${on_drop.bind(this)}
                    >
                    <div class="view" class=${classMap({["dropside-before"]: this.dropside === DropSide.Before, ["dropside-after"]: this.dropside === DropSide.After})}>
                        <div class="dragicon" @mouseover=${on_dragicon_hover.bind(this)}>☰</div>
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