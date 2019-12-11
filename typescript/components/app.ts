import { common_css } from "@styles/common";
import { customElement, LitElement, css} from "lit-element";
import { html } from "lit-html";

@customElement("todo-app")
class App extends LitElement {
    static get styles() { return styles() }

    render() {
        return html`
            <section class="todoapp">
                <todo-header id="header"></todo-header>
                <todo-main id="main"></todo-main>
                <todo-footer id="footer"></todo-footer>
            </section>
            <footer class="info">
                <p>Double-click to edit a todo</p>
                <p><a href="https://github.com/dakom/todo-shipyard-lit"><u>Repo on Github</u></a></p>
            </footer>
        `
    }
}

function styles() {
    return css`
        .todoapp {
            background: #fff;
            margin: 130px 0 40px 0;
            position: relative;
            box-shadow: 0 2px 4px 0 rgba(0, 0, 0, 0.2),
                        0 25px 50px 0 rgba(0, 0, 0, 0.1);
        }

        .info {
            margin: 65px auto 0;
            color: #4d4d4d;
            font-size: 11px;
            text-shadow: 0 1px 0 rgba(255, 255, 255, 0.5);
            text-align: center;
        }

        .info p {
            line-height: 1;
        }

        .info a {
            color: inherit;
            text-decoration: none;
            font-weight: 400;
        }

        .info a:hover {
            text-decoration: underline;
        }
    `
}
export default () => {
}