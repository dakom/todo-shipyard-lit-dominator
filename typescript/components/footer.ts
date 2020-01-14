import common_css from "@styles/common.css";
import footer_css from "@styles/footer.css";
import { customElement, LitElement, property, css} from "lit-element";
import {classMap} from 'lit-html/directives/class-map';
import {nothing} from "lit-html";
import { html } from "lit-html";
import {Filter} from "@components/types/types";
import {ClearCompleted} from "@events/events";

@customElement("todo-footer")
class Footer extends LitElement {
    static styles = [common_css, footer_css];

    @property( { type : Number} ) total = 0; 
    @property( { type : Number} ) remaining = 0; 
    @property( { type : Number} ) completed = 0; 
    @property( { type : Number} ) filter = Filter.All 

    render() {
        const {total, remaining, completed, filter} = this;

        const on_clear_completed = () => this.dispatchEvent(new ClearCompleted());

        return total === 0
            ? html`${nothing}`
            : html`
                <footer class="footer">
                    <span class="todo-count">${format_remaining(remaining)}</span>
                    <ul class="filters">
                        ${filterLine (Filter.All) (filter)}
                        ${filterLine (Filter.Active) (filter)}
                        ${filterLine (Filter.Completed) (filter)}
                    </ul>
                    ${completed ? html`<button class="clear-completed" @click=${on_clear_completed.bind(this)}>Clear completed</button>` : nothing}
                </footer>
            `;
    }
}

const filterLine = (filter:Filter) => (current:Filter) => {
    const [href, label] = {
        [Filter.All]: ["", "All"],
        [Filter.Active]: ["active", "Active"],
        [Filter.Completed]: ["completed", "Completed"],
    }[filter];

    const classes = classMap({selected: filter === current});

    return html`
        <li><a href="#/${href}" class=${classes}>${label}</a></li>
    `;
}
const format_remaining = (count:number) => html`${count} item${count !== 1 ? 's' : ''} left`;
