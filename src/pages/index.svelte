
<script lang="ts">
    import Background from '../assets/Background.png'
    import { goto } from '@roxi/routify';
    import { open } from '@tauri-apps/api/dialog';
    import { invoke } from '@tauri-apps/api/tauri';
    import { appWindow } from '@tauri-apps/api/window';

    import { launcher_settings } from '../lib/store';

    import OrangeButton from '../lib/_OrangeButton.svelte';

    let can_launch = false;
    let battle_eye = $launcher_settings?.battle_eye ?? false;
    let path: string = $launcher_settings?.path ?? "";

    if ($launcher_settings != undefined) {
        can_launch = true;
    }

    interface ConanLaunchSettings {
        battle_eye: boolean;
        continue_playing: boolean;
    }

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

    function launch(conan_launch_settings: ConanLaunchSettings) {

        invoke("launch_game", { launcherSettings: $launcher_settings, conanLaunchSettings: conan_launch_settings }).then((_) => {
            $goto("/chat");
        })

    }

    function launch_game() {
        
        let conan_launch_settings: ConanLaunchSettings = {
            battle_eye: battle_eye,
            continue_playing: false
        };

        launch(conan_launch_settings);

    }

    function continue_game() {

        let conan_launch_settings: ConanLaunchSettings = {
            battle_eye: battle_eye,
            continue_playing: true
        };

        launch(conan_launch_settings);

    }

    function check_for_hook() {

        invoke("is_hooked_in").then((hooked: any) => {

            console.log(hooked);
            let t_hooked = hooked as boolean;
            if (t_hooked) {
                $goto("/chat");
            }

        });

    }

    check_for_hook();

</script>

<img src={Background} class="absolute inset-0 w-full h-full object-cover z-0" alt="Background" />
<div class="absolute container z-10">

    <div class="h-64"></div>
    <div class="flex flex-row items-center justify-center gap-1">
        <input type="text" bind:value={path} placeholder="Path" class="rounded-md border px-1 w-96 bg-white outline-indigo-500 shadow" disabled/>
        <button type="button" class="rounded border px-2 bg-white hover:bg-slate-100 shadow" on:click={set_path}>Set Path</button>
    </div>
    <div class="h-4"></div>
    <div class="flex flex-row items-center justify-center gap-4">
        {#if can_launch}
            <OrangeButton text={"Launch"} on:click={launch_game}/>
            <OrangeButton text={"Continue"} on:click={continue_game}/>
        {/if}
    </div>

</div>

<div class="flex flex-row gap-2 absolute right-3 bottom-3 text-lg bg-neutral-900 rounded-md px-2 shadow-xl">
    <label class="text-white" for="battle-eye">Run with Battle-Eye</label>
    <input type="checkbox" name="battle-eye" class="w-5" bind:value={battle_eye}/>
</div>