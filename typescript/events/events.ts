export enum BridgeEvent {
    ChangePage,
    AddTodo,
    UpdateTodo,
    RemoveTodo,
}

/*
    Every type must be defined as a 2-element tuple
    First element is the event enum
    Second element is the event data
    It's like this instead of function args so that they can be associated
    And type checked strictly at compile-time
*/

export enum Page {
    Init
}

type ValidEvent = 
    [BridgeEvent.ChangePage, Page]
    | [BridgeEvent.AddTodo, string]
    | [BridgeEvent.RemoveTodo, number]

//this is loosely defined because the types are converted on the rust side 
type RustEventSender = (event_queue_ptr:number, evt_type:number, evt_data:any) => unknown;
let event_queue_ptr:number;
let send_event_to_rust:RustEventSender;

export const send_event = (event:ValidEvent) => {
    send_event_to_rust(event_queue_ptr, event[0], event[1]);
}

export const register_event_sender = (_event_queue_ptr:number) => (_send_event_to_rust:RustEventSender) => {
    event_queue_ptr = _event_queue_ptr;
    send_event_to_rust = _send_event_to_rust;

    send_event([BridgeEvent.ChangePage, Page.Init]);
    send_event([BridgeEvent.ChangePage, Page.Init]);
    send_event([BridgeEvent.AddTodo, "temp test!"]);
    send_event([BridgeEvent.RemoveTodo, 0]);
}