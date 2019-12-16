import {Filter} from "@components/types/types";
import {send_event, BridgeEvent} from "@events/events";

export const start_router = () => {
    window.addEventListener('hashchange', () => {
        send_event([BridgeEvent.FilterChange, get_filter()]);
    });

    send_event([BridgeEvent.FilterChange, get_filter()]);
}

const get_filter = ():Filter => {
    const page = location.hash.substr(2);
    return page === "active" ? Filter.Active
        : page === "completed" ? Filter.Completed
        : Filter.All
}