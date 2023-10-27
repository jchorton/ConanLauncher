// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use install_dir::InstallDir;

mod install_dir;
mod conan;

fn main() {

    if let Some(install_dir) = valid_existing_install() {
        install_dir.into_conan().launch_game();
        return;
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![valid_path, launch_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}

fn valid_existing_install() -> Option<InstallDir> {

    if let Some(install_dir) = InstallDir::from_file() {

        if install_dir.valid() {
            return Some(install_dir);
        }

    }
    None

}

#[tauri::command]
fn valid_path(path: String) -> bool {

    let install_dir = InstallDir::new(path);

    if install_dir.valid() {
        install_dir.save();
        return true;
    }

    false

}

#[tauri::command]
fn launch_game() {

    if let Some(install_dir) = InstallDir::from_file() {
        install_dir.into_conan().launch_game();
    }
    
}