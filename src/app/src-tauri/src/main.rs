// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use global_shortcuts::GlobalShortcuts;
use log::{error, info};
use notify::{RecursiveMode, Watcher};
use plugit::PluginManager;
use std::path::Path;
use std::sync::Arc;
use std::{
    fs::{self, File},
    io::Read,
};
use tauri::{Manager, Window};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};
use tauri_plugin_log::{LogLevel, Target, TargetKind, WEBVIEW_TARGET};

mod file_watcher;
mod global_shortcuts;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn paths() -> &'static [u8] {
    const DLL_PATH: &str =
        r"C:\Users\bubbl\Documents\sumit-app\src\plugins\Finder\bin\Debug\net8.0\Finder.dll";

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

    // let mut watcher = notify::recommended_watcher(|res| {
    //     match res {
    //        Ok(event) => println!("event: {:?}", event),
    //        Err(e) => println!("watch error: {:?}", e),
    //     }
    // }).unwrap();

    // // Add a path to be watched. All files and directories at that path and
    // // below will be monitored for changes.
    // watcher.watch(Path::new(r"C:\Users\bubbl\Documents\sumit-app\src\plugins"), RecursiveMode::Recursive).unwrap();

    let app = tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("main").unwrap();

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

    let mut file_watcher =
        file_watcher::ExtensionWatcher::new(&handle).expect("Failed to watch extensions");

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
