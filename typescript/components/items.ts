import {LitElement, html, css, customElement, property} from "lit-element";
import {common_css} from "@styles/common";
import {send_event, BridgeEvent} from "@events/events";
const ENTER_KEY = 13;

export class ItemsContainer extends LitElement {
    static get styles() {
        return common_css;
    }

    render() {
        return html`
            <slot></slot>
        `
    }
}

export class ItemContainer extends LitElement {
    static get styles() {
        return common_css;
    }

    @property( { type : String }  ) label = '';
    

    render() {
        return html`
            <h1>${this.label}</h1>
        `
    }
}
