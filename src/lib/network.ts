
import { writable } from "svelte/store";
import { listen } from '@tauri-apps/api/event'
import { invoke } from "@tauri-apps/api";

export interface INewMessage {
    message: string,
    sender: string
}

export const hooked_in = writable(false);
export const messages  = writable<INewMessage[]>([]);

export function init_network() {

    invoke("start_conan_hook_loop");
    invoke("start_webserver");

    listen("conan_hooked_in", (event: any) => {
        hooked_in.set(event.payload.hooked_in);
    });

    listen("webhook_message", (event: any) => {

        console.log(event);
        let new_messages = event.payload as INewMessage[];
        messages.update((t_messages) => {
            t_messages.push(...new_messages);
            return t_messages;
        });

    });

}