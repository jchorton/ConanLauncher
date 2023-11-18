
<script lang="ts">
    import { goto } from "@roxi/routify";
    import { characters } from "../../lib/characters";
    import OrangeButton from "../../lib/_OrangeButton.svelte";

    function on_new() {
        $goto("/edit_character");
    }

    function on_back() {
        $goto("/");
    }

</script>

<div class="container z-10 absolute">
    <div class="absolute left-2 top-2">
        <OrangeButton text={"Back"} on:click={on_back}/>
    </div>
    <div class="absolute right-0 top-2">
        <OrangeButton text={"New"} on:click={on_new}/>
    </div>
    <div class="flex flex-col">
        {#each $characters as character}
            <div>{character.name}</div>
        {/each}
    </div>
</div>
