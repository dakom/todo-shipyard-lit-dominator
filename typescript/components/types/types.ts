export interface Item {
    label: string;
    complete: boolean;
    id: EntityId;
}

export type EntityId = [number, number];

export enum Filter {
    All,
    Active,
    Completed
}

export enum KEYS {
    ENTER = 13,
    ESC = 27,
}