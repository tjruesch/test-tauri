#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use std::fs::File;
use std::fs::create_dir_all;
use std::io::prelude::*;
use std::vec;


fn read_file(f: String) -> String {
    let filename = f.to_string();

    let mut file = File::open(filename)
        .expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    return data;
}

fn write_file(filename: String, data: String) -> String {
    let mut file = File::create(filename)
        .expect("File not found");
    file.write_all(data.as_bytes())
        .expect("Error writing to file");
    return data;
}

#[tauri::command]
fn read_app_config() -> String {
    let home = std::env::var("HOME").unwrap();
    let path = home + "/.tinyconfig";
    let mut data = read_file(path);
    data.retain(|c| !c.is_whitespace());
    
    return data
}


#[tauri::command]
fn update_app_config(config: String) -> String {
    let home = std::env::var("HOME").unwrap();
    let path = home + "/.tinyconfig";
    write_file(path, config.clone());
    return config
}

#[tauri::command]
fn create_folder(folder: String) {
    create_dir_all(folder)
        .expect("Error creating folder");
}

fn main() {
    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_app_config, update_app_config, create_folder])
        .menu(
            tauri::Menu::os_default("Tiny Tools")
        )
        .run(ctx)
        .expect("error while running tauri application");
}
