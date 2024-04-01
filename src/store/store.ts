import create from "vue-zustand";
import { invoke } from "@tauri-apps/api/tauri";
//Types
import type { State } from "../types/State";
import type { Result } from "../types/Result";

export const useStore = create<State>((set, get) => ({}));
