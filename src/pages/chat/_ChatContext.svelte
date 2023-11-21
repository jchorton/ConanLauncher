
<script lang="ts">
    import { afterUpdate } from "svelte";
    import OrangeButton from "../../lib/_OrangeButton.svelte";
    import { messages, type INewMessage } from "../../lib/network";

    let context_box: HTMLElement;

    function on_clear() {
        $messages = [];
    }

    function get_altered_message(message: INewMessage): string {

        let idx: number | undefined = message.message.indexOf(message.sender);

        if (idx == undefined || idx > 0) {
            return message.message;
        }

        return message.message.replace(message.sender, "");

    }

    afterUpdate(() => {

        if ($messages.length > 0) {
            context_box.scrollTop = context_box.scrollHeight;
        }
        
    });

</script>

<div class="flex flex-col gap-2">
    <div class="flex flex-col gap-2 bg-white w-full rounded-2xl h-80 border-4 border-amber-900 overflow-y-auto" bind:this={context_box}>
        {#each $messages as message}
            <div>{message.sender}: {get_altered_message(message)}</div>
        {/each}
    </div>
    <OrangeButton text={"Clear Context"} on:click={on_clear} />
</div>