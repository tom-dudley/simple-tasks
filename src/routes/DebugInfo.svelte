<script>
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    let info = $state("");

    onMount(() => {
        async function getDebugInfo() {
            info = await invoke("debug_info");
        }

        getDebugInfo();
        return () => console.log("Got debug info!");
    });
</script>

<div>Debug info:{info}</div>
