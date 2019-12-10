import {LitElement, html, css, customElement} from "lit-element";
import {common_css} from "@styles/common";
import {send_event, BridgeEvent} from "@events/events";
const ENTER_KEY = 13;

export class ItemsContainer extends LitElement {
    static get styles() {
        return common_css;
    }

    render() {
        return html`
            <h1> ITEMS HERE!!! </h1>
        `
    }
}
