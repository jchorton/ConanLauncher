
import { writable, get } from "svelte/store";
import { listen } from '@tauri-apps/api/event'
import { invoke } from "@tauri-apps/api";

import { characters } from "./characters";
import { character_id } from "../pages/chat/chat_store";

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

        let new_messages = event.payload as INewMessage[];

        let t_character_id = get(character_id);
        let t_characters   = get(characters);

        if (t_characters != undefined) {

            let character_name = t_characters.find((character) => {
                return character.character_id == t_character_id;
            })!.name;

            new_messages = new_messages.filter((message) => {
                return message.sender != character_name;
            });

        }

        messages.update((t_messages) => {
            t_messages.push(...new_messages);
            return t_messages;

        });

    });

}