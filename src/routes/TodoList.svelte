<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    type AppState = {
        tasks: Task[];
        next_task_id: number;
    };

    type Task = {
        id: number;
        description: string;
    };

    let initialState: AppState = {
        tasks: [],
        next_task_id: 0,
    };

    let newTaskDescription = $state("");
    let appState: AppState = $state({
        tasks: [],
        next_task_id: 1,
    });

    function preventDefault(fn) {
        return function (event) {
            event.preventDefault();
            fn.call(this, event);
        };
    }

    async function loadTasks() {
        let state: AppState = await invoke("restore_app_state");
        appState = state;
    }

    const addTask = async (event) => {
        let task: Task = await invoke("add_task", {
            taskDescription: newTaskDescription,
        });

        // TODO: We get a Svelte error about mutating a value outside a component
        // console.log("Response from rust: " + JSON.stringify(task, null, 2));

        appState.tasks.push(task);

        event.target.reset();
    };

    async function removeTaskById(id: number) {
        appState.tasks = appState.tasks.filter((task) => task.id !== id);
        await invoke("remove_task", { taskId: id });
    }
</script>

<form onsubmit={preventDefault(addTask)}>
    <input
        id="task-input"
        type="text"
        autocomplete="off"
        spellcheck="false"
        placeholder="Task..."
        bind:value={newTaskDescription}
        onsubmit={addTask}
    />
    <button>Add</button>
</form>

<button onclick={loadTasks}>Load Tasks</button>

<ul>
    {#each appState.tasks as task (task.id)}
        <li>
            <div>{task.description}</div>
            <span
                ><button onclick={() => removeTaskById(task.id)}>Remove</button
                ></span
            >
        </li>
    {/each}
</ul>

<style>
    li {
        text-align: left;
        display: flex;
        align-items: center;
        justify-content: space-between;
        border-bottom: grey;
        border-bottom-style: solid;
        border-bottom-width: 1px;
        padding-top: 20px;
        padding-bottom: 20px;
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
