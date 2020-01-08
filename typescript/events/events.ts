export class AddTodo extends CustomEvent<{label: string}> {
    constructor(label:string) {
        super("add-todo", {
            detail: {label}
        });
    }
}