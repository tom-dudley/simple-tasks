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
        <i class="fa fa-thumbtack"></i>
        {#if alwaysOnTop}
            <div>Enabled</div>
        {:else}
            <div>Disabled</div>
        {/if}
    </button>
</div>

<style>
    button {
        border-radius: 8px;
        border: 1px solid transparent;
        padding: 0.6em 1.2em;
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
