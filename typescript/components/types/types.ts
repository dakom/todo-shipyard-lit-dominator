export interface Item {
    label: string;
    completed: boolean;
    id: string;
}

export enum Filter {
    All,
    Active,
    Completed
}