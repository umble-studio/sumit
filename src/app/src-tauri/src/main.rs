// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use extension::manager::ExtensionManager;
use global_shortcuts::GlobalShortcuts;
use std::sync::Arc;
use std::{
    fs::File,
    io::Read,
};
use tauri::{Manager, Window};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};
use tauri_plugin_log::{Target, TargetKind};

mod global_shortcuts;
mod watcher;
pub mod dotnet;
pub mod extension;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn paths() -> &'static [u8] {
    const DLL_PATH: &str =
        r"C:\Users\bubbl\Documents\sumit-app\src\plugins\Finder\client\bin\Debug\net8.0\Finder.dll";

    let mut file = File::open(DLL_PATH).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    Box::leak(buffer.into_boxed_slice())

    // let buffer = fs::read(DLL_PATH).unwrap();
    // let buffer = buffer.as_slice();
    // buffer.clone()
}

fn main() {
    let mut global_shortcuts = GlobalShortcuts::new();
    global_shortcuts.add_shortcut(
        Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN),
        |window| switch_window_visibility(window),
    );

    let app = tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("main").unwrap();

            // Only include this code on debug builds
            #[cfg(debug_assertions)]
            {
                let webview = window.get_webview_window("main").unwrap();
                webview.open_devtools();
            }

            let window = Arc::new(window);
            global_shortcuts.register(&app, window)?;

            Ok(())
        })
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, paths])
        .build(tauri::generate_context!())
        .expect("error while running tauri builder");

    let handle = app.handle();

    let file_watcher =
        watcher::ExtensionWatcher::new(&handle).expect("Failed to watch extensions");

    let extension_manager = ExtensionManager::new(&handle);

    app.run(|app_handle, event| {});
}

fn switch_window_visibility(window: &Window) -> Result<(), Box<dyn std::error::Error>> {
    if window.is_visible()? {
        window.hide()?;
    } else {
        window.show()?;
        window.set_focus()?;
    }
    Ok(())
}
