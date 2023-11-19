

<script lang="ts">

    import { characters } from "../../lib/characters";
    import { character_id } from "./chat_store";

</script>

<div class="flex flex-col gap-2">
    <label class="text-center text-white text-xl bg-amber-900 rounded-xl" for="character_select">Character</label>
    <select name="character_select" bind:value={$character_id} class="pl-3 pr-6 text-base border-amber-900 border-4 placeholder-gray-600 rounded-lg appearance-none focus:shadow-outline">
        <option disabled selected value="">Select a character</option>
        {#each $characters as character (character.character_id)}
            <option value={character.character_id}>{character.name}</option>
        {/each}
    </select>
</div>
