
import { writable, get } from "svelte/store";
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
var initialized: boolean = false;

export function init_characters() {

    if (initialized) {
        return;
    }

    initialized = true;

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

export function get_character(character_id: number): ICharacter | undefined {

    return get(characters).find((character) => {
        return character.character_id === character_id;
    });

}