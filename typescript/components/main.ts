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

    render() {
        const {items} = this;
        return html`
            ${items.length === 0 
                ? nothing
                : html`
                    <section class="main">
                        <input id="toggle-all" class="toggle-all" type="checkbox">
                        <label for="toggle-all">Mark all as complete</label>
                        <todo-list id="list" .items=${items}></todo-list>
                    </section>
                `
            }
        `
    }
}
