export interface Item {
    label: string;
    completed: boolean;
}

export enum Filter {
    All,
    Active,
    Completed
}