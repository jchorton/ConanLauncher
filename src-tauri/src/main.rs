// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use conan_launch_settings::ConanLaunchSettings;
use launcher_settings::LauncherSettings;

mod launcher_settings;
mod conan;
mod conan_hook;
mod conan_launch_settings;
mod database;
mod utils;
mod webhook;

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

fn main() {
    
    database::init().unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            valid_path,
            launch_game, 
            get_launcher_settings, 
            conan_hook::start_typing_loop,
            conan_hook::submit_actual_post,
            conan_hook::is_hooked_in,
            conan_hook::force_stop_loop,
            conan_hook::start_conan_hook_loop,
            database::character::get_characters,
            database::character::add_character,
            webhook::start_webserver
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}

#[tauri::command]
fn get_launcher_settings() -> Option<LauncherSettings> {

    if let Some(launcher_settings) = LauncherSettings::from_file() {

        if launcher_settings.valid() {
            return Some(launcher_settings);
        }

    }
    None

}

#[tauri::command]
fn valid_path(path: String) -> bool {

    let install_dir = LauncherSettings::new(path);

    if install_dir.valid() {
        install_dir.save();
        return true;
    }

    false

}

#[tauri::command]
fn launch_game(launcher_settings: LauncherSettings, conan_launch_settings: ConanLaunchSettings) {

    launcher_settings.save();
    launcher_settings.into_conan().launch_game(conan_launch_settings);
    
}