

<script lang="ts">

    import { invoke } from '@tauri-apps/api';
    import { fly } from 'svelte/transition';

    import { hooked_in } from '../../lib/network';
    import Background from '../../assets/Background.png'
    import OrangeButton from '../../lib/_OrangeButton.svelte';
    import { goto } from '@roxi/routify';

    let text_area: HTMLTextAreaElement;
    let text = "";
    let text_looping: boolean = false;
    let show_confirmation: boolean = false;

    const MAX_CHARACTERS_PER_POST = 2048;

    function on_submit() {
        show_confirmation = true;
    }

    function on_confirm() {

        show_confirmation = false;
        invoke("force_stop_loop").then(() => {

            text_looping = false;
            invoke("submit_actual_post", { post: text }).then(() => {
                text = "";
            });

        });

    }

    function on_cut() {

        window.navigator.clipboard.writeText(text);
        text = "";

    }

    function on_reset() {

        invoke("force_stop_loop").then(() => {

            text_looping = true;
            invoke("start_typing_loop").then(() => {});

        });

    }

    function on_stop() {

        invoke("force_stop_loop").then(() => {});
        text_looping = false;

    }

    function on_input() {

        if (text_looping) {
            return;
        }

        text_looping = true;
        invoke("start_typing_loop").then(() => {});

    }

    function on_back() {
        $goto("/");
    }
    
    $: if (!$hooked_in) {
        $goto("/");
    }

</script>

<img src={Background} class="absolute inset-0 w-full h-full object-cover z-0" alt="Background" />
<div class="absolute left-4 top-4 z-20">
    <OrangeButton text="Back" on:click={on_back} />
</div>
<div class="absolute container w-full h-full z-10 flex flex-col justify-center items-center">
    <textarea class="w-full h-80 ml-6 p-1 outline-none rounded-lg shadow-2xl border-orange-900 border-4" bind:this={text_area} bind:value={text} on:input={on_input}></textarea>
    <div class="text-white text-2xl ml-auto">{text.length}/{MAX_CHARACTERS_PER_POST}</div>
    <div class="h-4"></div>
    <div class="flex flex-row gap-2">
        <OrangeButton text="Reset" on:click={on_reset}/>
        <OrangeButton text="Stop" on:click={on_stop} />
        <OrangeButton text="Cut" on:click={on_cut} />
        <OrangeButton text="Submit" on:click={on_submit}/>
    </div>
    {#if show_confirmation}
        <div class="absolute container z-20 bg-orange-900 shadow-2xl w-64 h-32 rounded-lg border-2 border-black" transition:fly|local="{{ y: -500 }}">
            <div class="h-10"></div>
            <div class="text-center text-white text-lg">Are you sure?</div>
            <div class="flex flex-row items-center justify-center gap-2 w-full">
                <OrangeButton text="Yes" on:click={on_confirm}/>
                <OrangeButton text="No" on:click={() => { show_confirmation = false; }}/>
            </div>
        </div>
    {/if}
</div>