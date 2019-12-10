import {MainPage} from "@components/page";
import {ItemsContainer} from "@components/items";

export const register_web_components = () => {
    customElements.define("main-page", MainPage);
    customElements.define("items-container", ItemsContainer);
}

