export class AddTodo extends CustomEvent<{label: string}> {
    constructor(detail: {label:string}) {
        super("add-todo", { detail });
    }
}

export class RemoveTodo extends CustomEvent<{id: string}> {
    constructor(detail: {id:string}) {
        super("remove-todo", { detail });
    }
}

export class ToggleTodo extends CustomEvent<{id: string, complete: boolean}> {
    constructor(detail:{id:string, complete: boolean}) {
        super("toggle-todo", { detail });
    }
}


export class ChangeTodo extends CustomEvent<{id: string, label: string}> {
    constructor(detail:{id:string, label:string}) {
        super("change-todo", { detail });
    }
}

export class ToggleAllTodos extends CustomEvent<{complete: boolean}> {
    constructor(complete: boolean) {
        super("toggle-all-todos", { detail: {complete} });
    }
}