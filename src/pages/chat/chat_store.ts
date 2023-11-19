

import { writable } from "svelte/store";

export enum ChatStyle {
    EarlyMedieval  = "Early Medieval",
    LateMedieval   = "Late Medieval",
    EarlyVictorian = "Early Victorian",
    LateVictorian  = "Late Victorian",
    Modern         = "Modern",
}

export enum Verbosity {
    Concise = "Concise",
    Normal  = "Normal",
    Verbose = "Verbose",    
}

export const character_id = writable<number | undefined>(undefined);
export const chat_style   = writable<ChatStyle | undefined>(undefined);
export const verbosity    = writable<Verbosity | undefined>(undefined);