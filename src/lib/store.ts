
import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";

interface LauncherSettings {
    path: string,
    battle_eye: boolean
}

export const launcher_settings = writable<LauncherSettings | undefined>();

function default_launcher_settings(): LauncherSettings {

    return {
        path: "",
        battle_eye: false
    }

}

export function init_launcher_settings() {

    invoke("get_launcher_settings").then((result) => {

        console.log(result);
        let t_result = result as LauncherSettings | undefined;

        if (t_result != undefined) {

            launcher_settings.set(t_result);

        } else {

            launcher_settings.set(default_launcher_settings());

        }

    });

}