use std::sync::Mutex;
use tauri::State;
use tauri::{Builder, Manager};

#[derive(Default)]
struct AppState {
    tasks: Vec<Task>,
    next_task_id: i32,
}

struct Task {
    id: i32,
    description: String,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn add_task(state: State<'_, Mutex<AppState>>, task: &str) -> String {
    let mut state = state.lock().unwrap();

    let next_task_id = state.next_task_id;
    state.next_task_id += 1;

    state.tasks.push(Task {
        id: next_task_id,
        description: task.to_string(),
    });

    let number_of_tasks = state.tasks.len();
    println!("Tasks ({number_of_tasks}):");
    for task in &state.tasks {
        let id = &task.id;
        let description = &task.description;
        println!("    {id}");
        println!("    {description}");
    }
    task.to_string()
}

#[tauri::command]
fn remove_task(state: State<'_, Mutex<AppState>>, task_id: i32) {
    println!("Removing task: '{task_id}'");
    let mut state = state.lock().unwrap();

    // TODO: Use a Hashmap for the tasks
    if let Some(index) = state.tasks.iter().position(|task| task.id == task_id) {
        state.tasks.remove(index);
    } else {
        println!("Task '{task_id}' could not be removed as it wasn't not found!")
    }

    let number_of_tasks = state.tasks.len();
    println!("Tasks ({number_of_tasks}):");
    for task in &state.tasks {
        let id = &task.id;
        let description = &task.description;
        println!("    {id}");
        println!("    {description}");
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![add_task, remove_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
