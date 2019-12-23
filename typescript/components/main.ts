import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html} from "lit-html";
import {repeat} from 'lit-html/directives/repeat';
import main_css from "@styles/main.css";
import {send_event, BridgeEvent} from "@events/events";
import {Item} from "@components/types/types";

@customElement("todo-main")
export class Items extends LitElement {
    static styles = main_css;

    @property( { type : Array }  ) items = [] as Array<Item>;
    @property( { type : Boolean }  ) all_completed = false;

    render() {
        const {items} = this;

        console.log(this.all_completed);

        return html`
            ${items.length === 0 
                ? nothing
                : html`
                    <section class="main">
                        <input id="toggle-all" class="toggle-all" type="checkbox" @change=${on_toggle_all} ?checked=${this.all_completed} />
                        <label for="toggle-all">Mark all as complete</label>
                        <todo-list id="list" .items=${items}></todo-list>
                    </section>
                `
            }
        `
    }
}

const on_toggle_all = evt => {
    send_event([BridgeEvent.SetCompletedAll, evt.target.checked]);
}