

<script lang="ts">

    import { invoke } from '@tauri-apps/api';
    import { goto } from '@roxi/routify';
    import { fly } from 'svelte/transition';

    import { hooked_in } from '../../lib/network';
    import OrangeButton from '../../lib/_OrangeButton.svelte';
    
    import ChatContext from './_ChatContext.svelte';
    import { character_id, chat_style, verbosity, prose_style } from './chat_store';
    import { messages } from '../../lib/network';
    import Dropdowns from './_Dropdowns.svelte';

    let text = "";
    let text_looping: boolean = false;
    let show_confirmation: boolean = false;

    const MAX_CHARACTERS_PER_POST = 2048;

    function on_submit() {
        show_confirmation = true;
    }

    function on_confirm() {

        show_confirmation = false;

        if ($character_id == undefined) {
            alert("Please select a character.");
            return;
        }

        invoke("force_stop_loop").then(() => {

            text_looping = false;

            let payload = {
                character_id: $character_id,
                message: text
            };

            invoke("submit_actual_post", { characterMessage: payload }).then(() => {
                text = "";
            });

        });

    }

    function on_cut() {

        let t_text = get_generate_text();
        t_text += text;
        window.navigator.clipboard.writeText(t_text);
        text = "";
        
    }

    function get_generate_text(): string {

        let t_text = "((Generate /me from the content below.";
        if ($chat_style != undefined) {
            t_text += ` The generated writing style should be: ${$chat_style}.`;
        }

        if ($verbosity != undefined) {
            t_text += ` The generated verbosity should be: ${$verbosity}.`;
        }

        if ($prose_style != undefined) {
            t_text += ` The generated prose style should be: ${$prose_style}.`;
        }

        t_text += "))\n"
        return t_text;

    }

    function on_cut_with_context() {

        let t_text = "";
        if ($messages.length > 0) {

            t_text += "((Context))\n"
            $messages.forEach((message) => {
                t_text += `${message.sender}: ${message.message}\n`;
            });
            t_text += "\n";
            $messages = [];

        }

        t_text += get_generate_text();
        t_text += text;

        window.navigator.clipboard.writeText(t_text);
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

<div class="absolute left-4 top-4 z-20">
    <OrangeButton text="Back" on:click={on_back} />
</div>
<div class="absolute container w-full h-full z-10 flex flex-col justify-center items-center">
    <Dropdowns/>
    <div class="h-2"></div>
    <div class="grid grid-cols-5 gap-4 w-full">
        <div class="col-span-3">
            <textarea class="w-full h-80 p-1 outline-none rounded-lg shadow-2xl border-orange-900 border-4" bind:value={text} on:input={on_input}></textarea>
        </div>
        <div class="col-span-2">
            <ChatContext/>
        </div>
    </div>
    <div class="text-white text-2xl ml-auto">{text.length}/{MAX_CHARACTERS_PER_POST}</div>
    <div class="h-4"></div>
    <div class="flex flex-row gap-2">
        <OrangeButton text="Reset" on:click={on_reset}/>
        <OrangeButton text="Stop" on:click={on_stop} />
        <OrangeButton text="Cut" on:click={on_cut}/>
        <OrangeButton text="Cut With Context" on:click={on_cut_with_context} />
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