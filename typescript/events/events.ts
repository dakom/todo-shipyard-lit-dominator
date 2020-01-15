import {EntityId} from "@components/types/types";

export class AddTodo extends CustomEvent<{label: string}> {
    constructor(detail: {label:string}) {
        super("add-todo", { detail });
    }
}

export class RemoveTodo extends CustomEvent<{id: EntityId}> {
    constructor(detail: {id:EntityId}) {
        super("remove-todo", { detail });
    }
}

export class ToggleTodo extends CustomEvent<{id: EntityId, complete: boolean}> {
    constructor(detail:{id:EntityId, complete: boolean}) {
        super("toggle-todo", { detail });
    }
}


export class ChangeTodo extends CustomEvent<{id: EntityId, label: string}> {
    constructor(detail:{id:EntityId, label:string}) {
        super("change-todo", { detail });
    }
}

export class ToggleAllTodos extends CustomEvent<{complete: boolean}> {
    constructor(complete: boolean) {
        super("toggle-all-todos", { detail: {complete} });
    }
}

export class ClearCompleted extends CustomEvent<null> {
    constructor() {
        super("clear-completed");
    }
}