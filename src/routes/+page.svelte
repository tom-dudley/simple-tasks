<script lang="ts">
    import PinButton from "./PinButton.svelte";
    import TodoList from "./TodoList.svelte";
    import "@fortawesome/fontawesome-free/css/all.min.css";
</script>

<main class="container">
    <PinButton />
    <h1>Todo List</h1>

    <p>Enter some Todos!</p>

    <TodoList />
</main>

<style>
    :root {
        font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
        font-size: 16px;
        line-height: 24px;
        font-weight: 400;

        color: #0f0f0f;
        background-color: #f6f6f6;

        font-synthesis: none;
        text-rendering: optimizeLegibility;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        -webkit-text-size-adjust: 100%;
    }

    .container {
        margin: 0;
        padding-top: 5vh;
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
    }

    h1 {
        text-align: center;
    }

    @media (prefers-color-scheme: dark) {
        :root {
            color: #f6f6f6;
            background-color: #2f2f2f;
        }
    }
</style>
