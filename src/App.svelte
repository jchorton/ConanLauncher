<script lang="ts">
  
    import { Router } from '@roxi/routify'
    import { routes } from "../.routify/routes";

    import { init_launcher_settings } from './lib/store';
    init_launcher_settings();

</script>

<main>
    <Router {routes} />
</main>

<style></style>
