import {MainPage} from "@components/page";
import {ItemsContainer, ItemContainer} from "@components/items";
import {TopInput} from "@components/input";

export const register_web_components = () => {
    customElements.define("main-page", MainPage);
    customElements.define("items-container", ItemsContainer);
    customElements.define("item-container", ItemContainer);
    customElements.define("top-input", TopInput);
}

