

<script lang="ts">

    import { invoke } from '@tauri-apps/api';
    import Background from '../../assets/Background.png'
    import OrangeButton from '../../lib/_OrangeButton.svelte';

    let text = "";
    let text_looping: boolean = false;

    function on_submit() {

    }

    function on_reset() {


        invoke("force_stop_loop").then(() => {
            invoke("start_typing_loop").then(() => {});
        });

    }

    function on_stop() {
        invoke("force_stop_loop").then(() => {});
    }

    function on_input() {

        if (text_looping) {
            return;
        }

    }

</script>

<img src={Background} class="absolute inset-0 w-full h-full object-cover z-0" alt="Background" />
<div class="absolute container z-10 flex flex-col justify-center items-center">
    <textarea class="w-full" bind:value={text} on:input={on_input}></textarea>
    <div class="h-4"></div>
    <div class="flex flex-row gap-2">
        <OrangeButton text="Reset" on:click={on_reset}/>
        <OrangeButton text="Stop" on:click={on_stop} />
        <OrangeButton text="Submit" on:click={on_submit}/>
    </div>
</div>