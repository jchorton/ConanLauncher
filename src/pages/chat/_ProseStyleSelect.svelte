<script lang="ts">
    import { ProseStyle, prose_style } from "./chat_store";

    let proseOptions: ProseStyle[] = Object.values(ProseStyle);
</script>

<div class="flex flex-col gap-2">
    <label class="text-center text-white text-xl bg-amber-900 rounded-xl" for="prose_select">Prose Style</label>
    <select bind:value={$prose_style} id="prose_select" class="pl-3 pr-6 text-base border-amber-900 border-4 placeholder-gray-600 rounded-lg appearance-none focus:shadow-outline">
        <option disabled selected value="">Select a prose style</option>
        {#each proseOptions as proseOption}
            <option value={proseOption}>{proseOption}</option>
        {/each}
    </select>
</div>
