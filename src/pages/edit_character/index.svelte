
<script lang="ts">
    import { goto } from "@roxi/routify";
    import OrangeButton from "../../lib/_OrangeButton.svelte";
    import { invoke } from "@tauri-apps/api";

    let name: string | undefined = ""

    function on_create() {

        if (name == undefined || name == "") {
            alert("Name cannot be empty")
            return
        }

        name = name.trim();
        invoke("add_character", { newCharacter: { name } }).then(() => {
            $goto("/characters");
        });

    }

</script>

<div class="container z-10 absolute">
    <div class="h-2"></div>
    <div class="flex flex-col gap-2 items-center">
        <input type="text" placeholder="Name" bind:value={name} class="rounded-2xl pl-2 outline-none text-2xl" />
        <OrangeButton text={"Create"} on:click={on_create} />
    </div>
</div>