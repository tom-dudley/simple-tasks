use std::env;
use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::sync::Mutex;
use tauri::Manager;
use tauri::State;

#[derive(Default, serde::Serialize, serde::Deserialize, Clone)]
struct AppState {
    tasks: Vec<Task>,
    next_task_id: i32,
}

fn get_path() -> String {
    #[cfg(target_os = "macos")]
    let path = "/Users/tom/.tasks";

    #[cfg(target_os = "android")]
    let path = "/data/data/com.simple_tasks.app/.tasks";
    //let path = "/data/user/0/com.simple_tasks.app/.tasks";

    println!("Path: {path}");

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

#[tauri::command]
fn debug_info(state: State<'_, Mutex<AppState>>) -> String {
    //env::current_dir().unwrap_or_default().display().to_string()
    let mut result = String::new();
    result.push_str("\n");

    // Print env vars
    //for (key, value) in env::vars() {
    //    result.push_str(&format!("{}={};", key, value));
    //    result.push_str("\n");
    //}

    // Print paths
    //let path = "/data/data/com.simple_tasks.app";
    //let paths = fs::read_dir(path).unwrap();
    //for path in paths {
    //    let name = path.unwrap().path().display().to_string();
    //    //println!("Name: {name}");
    //    result.push_str(";");
    //    result.push_str(name.as_str());
    //    result.push_str("\n");
    //}

    result.clone()
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

#[cfg(target_os = "macos")]
fn hide_menus_on_mac(app: &mut tauri::App) {
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
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            println!("Starting app..");
            #[cfg(target_os = "macos")]
            hide_menus_on_mac(app);

            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            add_task,
            remove_task,
            restore_app_state,
            debug_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
