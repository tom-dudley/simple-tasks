<script>
    import { getCurrentWindow } from "@tauri-apps/api/window";

    let alwaysOnTop = $state(false);

    async function toggleAlwaysOnTop() {
        alwaysOnTop = !alwaysOnTop;
        await getCurrentWindow().setAlwaysOnTop(alwaysOnTop);
    }
</script>

<div style="display: flex; justify-content: flex-end">
    <button onclick={toggleAlwaysOnTop}>
        {#if alwaysOnTop}
            <i style="color: #ff5733" class="fa fa-thumbtack"></i>
            <div id="button-text">Enabled</div>
        {:else}
            <i class="fa fa-thumbtack"></i>
            <div id="button-text">Disabled</div>
        {/if}
    </button>
</div>

<style>
    .fa {
        font-size: 0.75em;
    }
    #button-text {
        font-size: 0.5em;
    }
    button {
        border-radius: 8px;
        border: 1px solid transparent;
        padding: 0.3em 0.5em;
        font-size: 1em;
        font-weight: 500;
        font-family: inherit;
        color: #0f0f0f;
        background-color: #ffffff;
        transition: border-color 0.25s;
        box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    }

    button {
        cursor: pointer;
    }

    button:hover {
        border-color: #396cd8;
    }
    button:active {
        border-color: #396cd8;
        background-color: #e8e8e8;
    }
    @media (prefers-color-scheme: dark) {
        input,
        button {
            color: #ffffff;
            background-color: #0f0f0f98;
        }
        button:active {
            background-color: #0f0f0f69;
        }
    }
</style>
