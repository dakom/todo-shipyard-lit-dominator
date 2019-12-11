import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html} from "lit-html";
import {repeat} from 'lit-html/directives/repeat';
import {common_css} from "@styles/common";
import {send_event, BridgeEvent} from "@events/events";
import {Item} from "@components/types/types";

@customElement("todo-main")
export class Items extends LitElement {
    static get styles() { return styles() }

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
                        <todo-list .items=${items}></todo-list>
                    </section>
                `
            }
        `
    }
}

function styles() {
        return css`
            .main {
                position: relative;
                z-index: 2;
                border-top: 1px solid #e6e6e6;
            }

            .toggle-all {
                width: 1px;
                height: 1px;
                border: none; /* Mobile Safari */
                opacity: 0;
                position: absolute;
                right: 100%;
                bottom: 100%;
            }

            .toggle-all + label {
                width: 60px;
                height: 34px;
                font-size: 0;
                position: absolute;
                top: -52px;
                left: -13px;
                -webkit-transform: rotate(90deg);
                transform: rotate(90deg);
            }

            .toggle-all + label:before {
                content: '❯';
                font-size: 22px;
                color: #e6e6e6;
                padding: 10px 27px 10px 27px;
            }

            .toggle-all:checked + label:before {
                color: #737373;
            }

        `;
    }
