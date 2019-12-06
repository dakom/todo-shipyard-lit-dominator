import {App} from "./app";

export const register_web_components = () => {
    customElements.define("my-app", App);
}