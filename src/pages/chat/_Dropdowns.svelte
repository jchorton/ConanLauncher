
<script lang="ts">

    import CharacterDropDown from "./_CharacterDropDown.svelte";
    import Styles from "./_Styles.svelte";
    import Verbosity from "./_Verbosity.svelte";
    import DialogueEra from "./_DialogueEra.svelte";
    import ProseStyleSelect from "./_ProseStyleSelect.svelte";

</script>

<div class="flex flex-row gap-2 mr-auto">
    <CharacterDropDown/>
    <Styles/>
    <!-- <ProseStyleSelect/> -->
    <Verbosity/>
    <DialogueEra/>
</div>