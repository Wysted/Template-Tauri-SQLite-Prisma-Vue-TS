import type { User } from "./User.ts";

export type State = {
    Users: User[];

    //Create
    createUser: (name: string) => Promise<void>;
    //Get
    getAllUser: () => Promise<void>;
    //Update
    updateUser: (id: number, name: string) => Promise<void>;
    //Delete
    deleteUser: (id: number) => Promise<void>;
};
