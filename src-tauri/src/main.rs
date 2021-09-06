#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn call_rust() {
  println!("Hello Tauri!");
}

#[tauri::command]
fn parse_file(path: String) -> String {
    let ret = format!("I have got your file {}", path);
    println!("receive from js: {}", ret);
    return ret;
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![call_rust, parse_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
