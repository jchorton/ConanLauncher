

<script lang="ts">

    import { characters } from "../../lib/characters";
    import { character_id } from "./chat_store";
    let selected_character: number = 0;

    function on_change() {
        $character_id = selected_character;
    }

</script>

<select bind:value={selected_character} on:change={on_change} class="pl-3 pr-6 text-base border-amber-900 border-4 placeholder-gray-600 rounded-lg appearance-none focus:shadow-outline">
    <option disabled selected value="">Select a character</option>
    {#each $characters as character (character.character_id)}
        <option value={character.character_id}>{character.name}</option>
    {/each}
</select>
