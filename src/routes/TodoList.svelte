<script lang="ts">
    let newTask = $state("");
    let nextId = 1;
    type Task = {
        id: number;
        task: string;
    };
    let tasks: Task[] = $state([]);

    function addTask() {
        tasks.push({
            id: nextId,
            task: newTask,
        });
        nextId += 1;
        // clearTextbox();
    }

    function removeTask() {
        tasks.pop();
    }
</script>

<form onsubmit={addTask}>
    <input
        id="task-input"
        type="text"
        autocomplete="off"
        spellcheck="false"
        placeholder="Task..."
        bind:value={newTask}
        onsubmit={addTask}
    />
    <button type="submit">Add</button>
</form>

<ul>
    {#each tasks as task (task.id)}
        <li>
            {task.task}
            <span><button onclick={removeTask}>Remove</button></span>
        </li>
    {/each}
</ul>

<style>
    li {
        text-align: left;
        display: flex;
        justify-content: space-between;
        border-bottom: grey;
        border-bottom-style: solid;
        border-bottom-width: 1px;
    }
    .row {
        display: flex;
        justify-content: center;
    }

    input,
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

    input,
    button {
        outline: none;
    }

    #task-input {
        margin-right: 5px;
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
