export interface Item {
    label: string;
    complete: boolean;
    id: string;
}

export enum Filter {
    All,
    Active,
    Completed
}

export enum KEYS {
    ENTER = 13,
    ESC = 27,
}