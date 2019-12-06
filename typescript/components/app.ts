import {LitElement, html} from "lit-element";

export class App extends LitElement {
    render() {
        const create_new = evt => {
            console.log(evt);
        }
        return html`
            <section class="todoapp">
                <header class="header">
                    <h1>todos</h1>
                    <input class="new-todo" @keydown=${create_new} placeholder="What needs to be done?" autofocus />
                </header>
            </section>
            <footer class="info">
                <p>Double-click to edit a todo</p>
            </footer>
        `
    }

    //using imported css from todo-mvc, so light dom
    createRenderRoot() {
        return this;
    }
}
