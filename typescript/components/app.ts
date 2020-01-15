import { customElement, LitElement, css} from "lit-element";
import { html } from "lit-html";
import app_css from "./styles/app.css";

@customElement("todo-app")
class App extends LitElement {
    static styles = app_css;

    render() {
        return html`
            <section class="todoapp">
                <slot></slot>
            </section>
            <footer class="info">
                <p>Double-click to edit a todo</p>
                <p><a href="https://github.com/dakom/todo-shipyard-lit-dominator"><u>Repo on Github</u></a></p>
            </footer>
        `
    }
}