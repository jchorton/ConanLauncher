
import { writable } from "svelte/store";
import { listen } from '@tauri-apps/api/event'
import { invoke } from "@tauri-apps/api";

export const hooked_in = writable(false);

export function init_network() {

    invoke("start_conan_hook_loop");

    listen("conan_hooked_in", (event: any) => {

        hooked_in.set(event.payload.hooked_in);

    });

}