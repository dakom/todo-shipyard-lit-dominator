//needs to first be bound to the component
//e.g. send_event.bind (this) ("foo-event", {foo: "bar"})
export function send_event(name:string, data:any) {
    this.dispatchEvent(new CustomEvent(name, {
        detail: data 
    }));
}