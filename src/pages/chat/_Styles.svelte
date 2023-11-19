
<script lang="ts">
    import { ChatStyle, chat_style } from "./chat_store";

    let options: ChatStyle[] = Object.values(ChatStyle);

</script>


<select bind:value={$chat_style} class="pl-3 pr-6 text-base border-amber-900 border-4 placeholder-gray-600 rounded-lg appearance-none focus:shadow-outline">
    <option disabled selected value="">Select a writing style</option>
    {#each options as option}
        <option value={option}>{option}</option>
    {/each}
</select>