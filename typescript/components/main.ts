import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html} from "lit-html";
import {repeat} from 'lit-html/directives/repeat';
import main_css from "@styles/main.css";
import {ToggleAllTodos} from "@events/events";

@customElement("todo-main")
export class Items extends LitElement {
    static styles = main_css;

    @property( { type : Number}  ) len = 0;
    @property( { type : Boolean }  ) all_completed = false;

    render() {

        const on_toggle_all = (evt:any) => this.dispatchEvent(new ToggleAllTodos(evt.target.checked));

        return html`
            ${this.len === 0 
                ? nothing
                : html`
                    <section class="main">
                        <input id="toggle-all" class="toggle-all" type="checkbox" @change=${on_toggle_all} .checked=${this.all_completed} />
                        <label for="toggle-all">Mark all as complete</label>
                        <todo-list id="list">
                            <slot></slot>
                        </todo-list>
                    </section>
                `
            }
        `
    }
}