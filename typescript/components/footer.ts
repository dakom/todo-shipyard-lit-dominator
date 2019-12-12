import { common_css } from "@styles/common";
import { customElement, LitElement, property, css} from "lit-element";
import {classMap} from 'lit-html/directives/class-map';
import {nothing} from "lit-html";
import { html } from "lit-html";
import {Filter} from "@components/types/types";

@customElement("todo-footer")
class Footer extends LitElement {
    static get styles() { return styles() }

    @property( { type : Number} ) count = 0; 
    @property( { type : Number} ) filter = Filter.All 
    render() {
        const {count, filter} = this;

        return count === 0
            ? html`${nothing}`
            : html`
                <footer class="footer">
                    <span class="todo-count">${formatCount(count)}</span>
                    <ul class="filters">
                        ${filterLine (Filter.All) (filter)}
                        ${filterLine (Filter.Active) (filter)}
                        ${filterLine (Filter.Completed) (filter)}
                    </ul>
                    <button class="clear-completed">Clear completed</button>
                </footer>
            `;
    }
}

const filterLine = (filter:Filter) => (current:Filter) => {
    const info = {
        [Filter.All]: ["", "All"],
        [Filter.Active]: ["active", "Active"],
        [Filter.Completed]: ["completed", "Completed"],
    };

    const [href, label] = info[filter];

    const classes = classMap({selected: filter === current});

    return html`
        <li><a href="#/${href}" class=${classes}>${label}</a></li>
    `;
}
const formatCount = (count:number) => html`${count} item${count !== 1 ? 's' : ''} left`;

function styles() {
        return [common_css, css`
            .footer {
                padding: 10px 15px;
                height: 20px;
                text-align: center;
                font-size: 15px;
                border-top: 1px solid #e6e6e6;
            }

            .footer:before {
                content: '';
                position: absolute;
                right: 0;
                bottom: 0;
                left: 0;
                height: 50px;
                overflow: hidden;
                box-shadow: 0 1px 1px rgba(0, 0, 0, 0.2),
                            0 8px 0 -3px #f6f6f6,
                            0 9px 1px -3px rgba(0, 0, 0, 0.2),
                            0 16px 0 -6px #f6f6f6,
                            0 17px 2px -6px rgba(0, 0, 0, 0.2);
            }

            .todo-count {
                float: left;
                text-align: left;
            }

            .todo-count strong {
                font-weight: 300;
            }

            .filters {
                margin: 0;
                padding: 0;
                list-style: none;
                position: absolute;
                right: 0;
                left: 0;
            }

            .filters li {
                display: inline;
            }

            .filters li a {
                color: inherit;
                margin: 3px;
                padding: 3px 7px;
                text-decoration: none;
                border: 1px solid transparent;
                border-radius: 3px;
            }

            .filters li a:hover {
                border-color: rgba(175, 47, 47, 0.1);
            }

            .filters li a.selected {
                border-color: rgba(175, 47, 47, 0.2);
            }


            .clear-completed,
            html .clear-completed:active {
                float: right;
                position: relative;
                line-height: 20px;
                text-decoration: none;
                cursor: pointer;
            }

            .clear-completed:hover {
                text-decoration: underline;
            }

            @media (max-width: 430px) {
                .footer {
                    height: 50px;
                }

                .filters {
                    bottom: 10px;
                }
            }

        `];
}