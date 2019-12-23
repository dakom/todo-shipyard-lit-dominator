import {Filter} from "@components/types/types";
export enum BridgeEvent {
    FilterChange,
    AddTodo,
    SetTodoCompleted,
    RemoveTodo,
    ClearCompleted
}

/*
    Every type must be defined as a 2-element tuple
    First element is the event enum
    Second element is the event data
    It's like this instead of function args so that they can be associated
    And type checked strictly at compile-time
*/

type ValidEvent = 
    [BridgeEvent.FilterChange, Filter]
    | [BridgeEvent.AddTodo, string]
    | [BridgeEvent.RemoveTodo, string]
    | [BridgeEvent.SetTodoCompleted, [string, boolean]]
    | BridgeEvent.ClearCompleted

//this is loosely defined because the types are converted on the rust side 
type RustEventSender = (event_queue_ptr:number, evt_type:number, evt_data:any) => unknown;
let rust_app_ctx_ptr:number;
let send_event_to_rust:RustEventSender;

export const send_event = (event:ValidEvent) => {
    if(Array.isArray(event)) {
        send_event_to_rust(rust_app_ctx_ptr, event[0], event[1]);
    } else {
        send_event_to_rust(rust_app_ctx_ptr, event, undefined);
    }
}

export const register_event_sender = (_rust_app_ctx_ptr:number) => (_send_event_to_rust:RustEventSender) => {
    rust_app_ctx_ptr= _rust_app_ctx_ptr;
    send_event_to_rust = _send_event_to_rust;

    send_event([BridgeEvent.AddTodo, "toggle check all when all items are selected"]);
    send_event([BridgeEvent.AddTodo, "(un)check all"]);
    send_event([BridgeEvent.AddTodo, "local storage"]);
}