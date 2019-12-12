import {Filter} from "@components/types/types";

export const start_router = () => {
    window.addEventListener('hashchange', () => {
        console.log(get_filter());
    });
}

const get_filter = ():Filter => {
    const page = location.hash.substr(2);
    return page === "active" ? Filter.Active
        : page === "completed" ? Filter.Completed
        : Filter.All
}