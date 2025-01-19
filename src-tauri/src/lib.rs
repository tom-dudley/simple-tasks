use std::fs;
use std::fs::File;
use std::sync::Mutex;
use tauri::State;
use tauri::{Manager, Window};

#[derive(Default, serde::Serialize, serde::Deserialize, Clone)]
struct AppState {
    tasks: Vec<Task>,
    next_task_id: i32,
}

fn get_path() -> String {
    let path = "/Users/tom/.tasks";
    path.to_string()
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct Task {
    id: i32,
    description: String,
}

fn save_tasks(state: &AppState) -> std::io::Result<()> {
    let path = get_path();
    let file = File::create(path)?;
    serde_json::to_writer(file, &state)?;
    Ok(())
}

#[tauri::command]
fn restore_app_state(state_mutex: State<'_, Mutex<AppState>>) -> AppState {
    println!("Restoring app state...");
    let path = get_path();

    let restore_state = read_state_from_file(path);
    let state_to_return = restore_state.clone();

    let number_of_tasks = restore_state.tasks.len();
    println!("Restored {number_of_tasks} tasks");

    let mut state = state_mutex.lock().unwrap();
    state.tasks = restore_state.tasks;
    state.next_task_id = restore_state.next_task_id;

    state_to_return
}

fn read_state_from_file(path: String) -> AppState {
    let data = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return AppState::default(),
    };

    serde_json::from_str(&data).unwrap_or_default()
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn add_task(state: State<'_, Mutex<AppState>>, task_description: &str) -> Task {
    let mut state = state.lock().unwrap();

    let next_task_id = state.next_task_id;
    state.next_task_id += 1;

    let new_task = Task {
        id: next_task_id,
        description: task_description.to_string(),
    };

    // TODO: Figure out if there's a better thing to do here than clone
    let task_to_return = new_task.clone();

    state.tasks.push(new_task);

    let number_of_tasks = state.tasks.len();
    println!("Tasks ({number_of_tasks}):");
    for task in &state.tasks {
        let id = &task.id;
        let description = &task.description;
        println!("    {id}");
        println!("    {description}");
    }

    match save_tasks(&state) {
        Ok(_) => println!("Saving completed successfully."),
        Err(e) => eprintln!("Error occurred: {}", e),
    }

    task_to_return
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

    match save_tasks(&state) {
        Ok(_) => println!("Saving completed successfully."),
        Err(e) => eprintln!("Error occurred: {}", e),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let window = app
                .get_webview_window("main")
                .expect("Could not get window");
            let menu = window.menu().expect("Could not get menu");

            for item in menu.items() {
                for thing in item {
                    let sub_menu = thing.as_submenu().unwrap();
                    let name = sub_menu.text().unwrap();

                    match name.as_str() {
                        "Edit" | "Window" | "View" | "Help" => {
                            println!("Removing menu item '{name}'");
                            menu.remove(&thing);
                        }
                        _ => println!("Retaining menu item '{name}'"),
                    }
                }
                println!("End of item")
            }

            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            add_task,
            remove_task,
            restore_app_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
