export interface Item {
    label: string;
    complete: boolean;
    id: EntityId;
}

export enum DropSide {
    None,
    Before,
    After,
}