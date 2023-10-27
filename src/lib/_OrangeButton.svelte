
<script lang="ts">
    import { createEventDispatcher } from "svelte";

    export let text: string;
    const dispatch = createEventDispatcher();

    function on_click() {
        dispatch("click");
    }

</script>

<button type="button" class="rounded border px-2 bg-orange-800 text-3xl text-white hover:bg-orange-900 border-none shadow" on:click={on_click}>{text}</button>
