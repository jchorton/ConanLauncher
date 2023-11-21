<script lang="ts">
    import { goto } from "@roxi/routify";
    import { characters, type ICharacter } from "../../lib/characters";
    import OrangeButton from "../../lib/_OrangeButton.svelte";

    function on_new() {
        $goto("/edit_character");
    }

    function on_back() {
        $goto("/");
    }

    function on_character_click(character: ICharacter) {
        //$goto(`/character/${character.character_id}`);
    }

</script>

<div class="container z-10 absolute">
    <div class="absolute left-2 top-2">
        <OrangeButton text={"Back"} on:click={on_back}/>
    </div>
    <div class="absolute right-2 top-2">
        <OrangeButton text={"New"} on:click={on_new}/>
    </div>
    <div class="h-12"></div>
    <div class="flex flex-col gap-2 mt-8 w-full items-center">
        {#each $characters as character}
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div class="px-4 py-2 bg-orange-800 text-white text-xl rounded shadow cursor-pointer w-96 text-center" on:click={() => on_character_click(character)}>
                {character.name}
            </div>
        {/each}
    </div>
</div>