<script lang="ts">
    import { goto } from "@roxi/routify";
    import OrangeButton from "../../lib/_OrangeButton.svelte";
    import ConanPlaceholder from '../../assets/conan_placeholder.png'
    import { invoke } from "@tauri-apps/api";

    let name: string | undefined = "";
    let description: string | undefined = "";
    let avatar: File | null = null;
    let avatarUrl: string | undefined = ConanPlaceholder;

    function on_create() {

        if (name === undefined || name.trim() === "") {
            alert("Name cannot be empty");
            return;
        }

        name = name.trim();

        let newCharacter = { name, description, image: avatar ? URL.createObjectURL(avatar) : undefined };

        invoke("add_character", { newCharacter }).then(() => {
            $goto("/characters");
        });

    }

    function on_file_change(event: any) {

        avatar = event.target.files[0];
        avatarUrl = avatar ? URL.createObjectURL(avatar) : ConanPlaceholder;

    }

    function trigger_file_upload() {

        const fileInput: HTMLElement | null = document.getElementById('fileInput');
        fileInput!.click();

    }
</script>

<div class="container z-10 absolute">
    <div class="h-2"></div>
    <div class="flex flex-col gap-2 items-center">
        <input id="fileInput" type="file" on:change={on_file_change} class="hidden" />
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div class="w-48 h-48 bg-cover bg-center cursor-pointer border-orange-800 border-4" style="background-image: url({avatarUrl});" on:click={trigger_file_upload}></div>
        <input type="text" placeholder="Name" bind:value={name} class="rounded-sm pl-2 outline-none text-2xl border-orange-800 border-4" />
        <textarea bind:value={description} placeholder="Description" class="px-3 py-2 placeholder-gray-500 w-1/2 h-64 text-gray-900 border-orange-800 border-4 rounded-sm focus:outline-none autoexpand" rows="2" />
        <OrangeButton text={"Create"} on:click={on_create} />
    </div>
</div>