import {css} from "lit-element";

export const common_css = css`
    :focus {
        outline: 0;
    }

    .hidden {
        display: none;
    }

    .new-todo,
    .edit {
        position: relative;
        margin: 0;
        width: 100%;
        font-size: 24px;
        font-family: inherit;
        font-weight: inherit;
        line-height: 1.4em;
        color: inherit;
        padding: 6px;
        border: 1px solid #999;
        box-shadow: inset 0 -1px 5px 0 rgba(0, 0, 0, 0.2);
        box-sizing: border-box;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
    }

    hr {
        margin: 20px 0;
        border: 0;
        border-top: 1px dashed #c5c5c5;
        border-bottom: 1px dashed #f7f7f7;
    }
`;