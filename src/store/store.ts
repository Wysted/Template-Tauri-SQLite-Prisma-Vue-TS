import create from "vue-zustand";
import { invoke } from "@tauri-apps/api/tauri";
//Types
import type { State } from "../types/State";
import type { User } from "../types/User";
import type { Result } from "../types/Result";

export const useStore = create<State>((set, get) => ({
    Users: [],
    createUser: async (name: string) => {
        const result: Result<User, string> = await invoke("create_user", {
            name,
        });
        if (typeof result === "string") {
            console.error(`Something went wrong: ${result}`);
            return;
        }
        set({
            Users: [...get().Users, result],
        });
    },
    getAllUser: async () => {
        const result: Result<User[], string> = await invoke("get_all_user");
        if (typeof result === "string") {
            console.error(`Something went wrong: ${result}`);
            return;
        }

        set({
            Users: result,
        });
    },
    updateUser: async (id: number, name: string) => {
        const result: Result<User, string> = await invoke("update_user", {
            id,
            name,
        });
        if (typeof result === "string") {
            console.error(`Something went wrong: ${result}`);
            return;
        }
        console.log(result);
        set({
            Users: [
                ...get().Users.map((user) => {
                    if (user.id === id) {
                        user = result;
                    }

                    return user;
                }),
            ],
        });
    },
    deleteUser: async (id: number) => {
        const result: Result<User, string> = await invoke("delete_user", {
            id,
        });
        if (typeof result === "string") {
            console.error(`Something went wrong: ${result}`);
            return;
        }
        set({
            Users: [
                ...get().Users.filter((user) => {
                    if (result.id != id) return user;
                }),
            ],
        });
    },
}));
