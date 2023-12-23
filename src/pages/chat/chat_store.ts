

import { writable } from "svelte/store";

export enum ChatStyle {
    ClassicalAntiquity = "Classical Antiquity",
    EarlyMedieval      = "Early Medieval",
    LateMedieval       = "Late Medieval",
    Renaissance        = "Renaissance",
    Enlightenment      = "Enlightenment",
    Romantic           = "Romantic",
    EarlyVictorian     = "Early Victorian",
    LateVictorian      = "Late Victorian",
    Modernist          = "Modernist",
    Postmodern         = "Postmodern",
    Contemporary       = "Contemporary",
}

export enum Verbosity {
    Concise = "Concise",
    Normal  = "Normal",
    Verbose = "Verbose",    
}

export enum ProseStyle {
    Plain = "Plain Prose",
    Descriptive = "Descriptive Prose",
    Purple = "Purple Prose",
    Technical = "Technical Prose",
    StreamOfConsciousness = "Stream of Consciousness",
}

export enum DialogueEra {
    OldEnglish        = "Old English",
    MiddleEnglish     = "Middle English",
    EarlyModernEnglish = "Early Modern English",
    ModernEnglish      = "Modern English",
}

export const character_id = writable<number | undefined>(undefined);
export const chat_style   = writable<ChatStyle | undefined>(undefined);
export const verbosity    = writable<Verbosity | undefined>(undefined);
export const prose_style  = writable<ProseStyle | undefined>(undefined);
export const dialogue_era = writable<DialogueEra | undefined>(undefined);
export const text_chat_looping = writable(false);