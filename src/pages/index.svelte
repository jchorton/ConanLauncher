
<script lang="ts">
    import { open } from '@tauri-apps/api/dialog';
    import { invoke } from '@tauri-apps/api/tauri';

    let path = "";
    async function set_path() {
        
        const selected = await open({
            multiple: false,
            directory: true
        });

        if (selected != null && typeof selected === 'string') {
            path = selected
        }

    }

</script>

<div class="container">

    <div class="flex flex-row items-center justify-center gap-1">
        <input type="text" bind:value={path} placeholder="Path" class="rounded-md border px-1 w-96 outline-indigo-500" disabled/>
        <button type="button" class="rounded border px-2 hover:bg-slate-100" on:click={set_path}>Set Path</button>
    </div>

</div>