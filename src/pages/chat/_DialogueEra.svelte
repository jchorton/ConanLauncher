
<script lang="ts">
    import { DialogueEra, dialogue_era } from "./chat_store";

    let options: DialogueEra[] = Object.values(DialogueEra);

</script>

<div class="flex flex-col gap-2">
    <label class="text-center text-white text-xl bg-amber-900 rounded-xl" for="character_select">Dialogue Era</label>
    <select bind:value={$dialogue_era} class="pl-3 pr-6 text-base border-amber-900 border-4 placeholder-gray-600 rounded-lg appearance-none focus:shadow-outline">
        <option disabled selected value="">Select Dialogue Era</option>
        {#each options as option}
            <option value={option}>{option}</option>
        {/each}
    </select>
</div>