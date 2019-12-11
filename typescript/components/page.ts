import { common_css } from "@styles/common";
import { customElement, LitElement } from "lit-element";
import { html } from "lit-html";
const ENTER_KEY = 13;

//TODO - enhance with unique id etc.?
type Item = string;

@customElement("main-page")
class MainPage extends LitElement {
    static get styles() {
        return common_css;
    }


    render() {
        return html`
            <section class="todoapp">
                <header class="header">
                    <h1>todos</h1>
                    <top-input></top-input> 
                </header>
            </section>
            <items-container id="items"></items-container>
            <footer class="info">
                <p>Double-click to edit a todo</p>
                <p><a href="https://github.com/dakom/todo-shipyard-lit"><u>Repo on Github</u></a></p>
            </footer>
        `
    }
}

export default () => {
}