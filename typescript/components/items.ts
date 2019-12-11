import {LitElement, customElement, property} from "lit-element";
import {nothing, html} from "lit-html";
import {repeat} from 'lit-html/directives/repeat';
import {common_css} from "@styles/common";
import {send_event, BridgeEvent} from "@events/events";
const ENTER_KEY = 13;

@customElement("items-container")
export class Items extends LitElement {
    static get styles() {
        return common_css;
    }

    @property( { type : Array }  ) items = [] as Array<Item>;

    render() {
        const {items} = this;

        return html`
            ${items.length === 0 
                ? nothing
                : html`
                        <div id="main">
                            ${repeat(
                                items, 
                                item => item,
                                item => html`<item-container>${item}</item-container>`
                            )}
                        </div>
                `
            }
        `
    }
}

@customElement("item-container")
export class Item extends LitElement {
    static get styles() {
        return common_css;
    }

    render() {
        return html`
            <h1><slot></slot></h1>
        `
    }
}

export default () => {
}