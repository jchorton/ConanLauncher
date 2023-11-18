
<script lang="ts">
    import { messages } from "../../lib/network";
</script>

<div class="flex flex-col gap-2 bg-white w-full rounded-2xl h-80 border-4 border-amber-900">
    {#each $messages as message}
        <div>{message.sender}: {message.message}</div>
    {/each}
</div>