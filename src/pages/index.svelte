
<script lang="ts">
    import Background from '../assets/Background.png'
    import { open } from '@tauri-apps/api/dialog';
    import { invoke } from '@tauri-apps/api/tauri';
    import { exit } from '@tauri-apps/api/process';

    let path = "";
    let can_launch = false;
    async function set_path() {
        
        const selected = await open({
            multiple: false,
            directory: true
        });

        let temp_path: string;

        if (selected != null && typeof selected === 'string') {
            temp_path = selected
        } else {
            return;
        }

        invoke("valid_path", { path: temp_path }).then((response) => {

            if (response) {

                can_launch = true;
                path = temp_path;

            } else {

                can_launch = false;
                alert("Invalid Path");

            }

        });

    }

    function launch_game() {
        
        if (!can_launch) {
            return;
        }

        invoke("launch_game").then((_) => {
            exit(0);
        })

    }

</script>

<img src={Background} class="absolute inset-0 w-full h-full object-cover z-0" alt="Background" />
<div class="absolute container z-10">

    <div class="h-64"></div>
    <div class="flex flex-row items-center justify-center gap-1">
        <input type="text" bind:value={path} placeholder="Path" class="rounded-md border px-1 w-96 bg-white outline-indigo-500" disabled/>
        <button type="button" class="rounded border px-2 bg-white hover:bg-slate-100" on:click={set_path}>Set Path</button>
    </div>
    <div class="h-4"></div>
    <div class="flex flex-row items-center justify-center gap-1">
        {#if can_launch}
            <button type="button" class="rounded border px-2 bg-orange-800 text-xl text-white hover:bg-orange-900 border-none" on:click={launch_game}>Launch Game</button>
        {/if}
    </div>

</div>