
import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";

export interface ICharacter {
    character_id: number;
    name: string;
    description?: string;
    image?: string;
}

export interface INewCharacter {
    name: string;
    description?: string;
    image?: string;
}


export const characters = writable<ICharacter[]>([]);

export function init_characters() {

    invoke("get_characters").then((result: any) => {
        let t_result = result as ICharacter[];
        characters.set(t_result);
    });

    listen("character_added", (event: any) => {

        let new_character = event.payload as ICharacter;
        characters.update((t_characters) => {

            t_characters.push(new_character);
            return t_characters;

        });

    });

}