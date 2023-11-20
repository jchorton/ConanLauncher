
<script lang="ts">

    import { onMount } from "svelte";

    export let text_looping: boolean = false;
    let time_elapsed: number = 0;
    let current_time = Date.now();

    onMount(() => {

        let interval = setInterval(() => {

            let diff = Date.now() - current_time;
            if (text_looping) {
                time_elapsed += diff;
            }
            current_time = Date.now();

        }, 500);

        return () => {
            clearInterval(interval);
        }

    });

    function convert_to_time_string(time: number): string {

        let minutes = Math.floor(time_elapsed/(1000 * 60));
        let seconds = Math.floor((time_elapsed % (1000 * 60)) / 1000).toString().padStart(2, "0");

        return `${minutes}:${seconds}`

    }

    $: if (!text_looping) {
        time_elapsed = 0;
    } else {
        current_time = Date.now();
    }

</script>

<div class="rounded-xl bg-amber-900 text-white text-xl px-2 shadow-xl">Post time: {convert_to_time_string(time_elapsed)}</div>
