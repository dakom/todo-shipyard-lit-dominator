export enum BridgeEvent {
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

type ValidEvent = 
    [BridgeEvent.AddTodo, string]
    | [BridgeEvent.RemoveTodo, number]

//this is loosely defined because the types are converted on the rust side 
type RustEventSender = (evt_type:number, evt_data:any) => unknown;
let send_event_to_rust:RustEventSender;

export const send_event = (event:ValidEvent) => {
    send_event_to_rust(event[0], event[1]);
}

export const register_event_sender = (_send_event_to_rust:RustEventSender) => {
    send_event_to_rust = _send_event_to_rust;

    send_event([BridgeEvent.AddTodo, "temp test!"]);
    send_event([BridgeEvent.RemoveTodo, 0]);
}