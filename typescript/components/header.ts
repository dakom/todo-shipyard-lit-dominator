import {LitElement, css, customElement, property} from "lit-element";
import {nothing, html} from "lit-html";
import {repeat} from 'lit-html/directives/repeat';
import {common_css} from "@styles/common";
import {send_event, BridgeEvent} from "@events/events";
const ENTER_KEY = 13;

@customElement("todo-header")
export class Items extends LitElement {
    static get styles() { return styles() }

    render() {
        return html`
            <header class="header">
                <h1>todos</h1>
                <todo-input id="input"></todo-input>
            </header>
        `;
    }
}

function styles() {
    return css`
            h1 {
                position: absolute;
                top: -140px;
                width: 100%;
                font-size: 80px;
                font-weight: 200;
                text-align: center;
                color: #b83f45;
                -webkit-text-rendering: optimizeLegibility;
                -moz-text-rendering: optimizeLegibility;
                text-rendering: optimizeLegibility;
            }
        `
}