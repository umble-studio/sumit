// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use global_shortcuts::GlobalShortcuts;
use log::{error, info};
use plugit::PluginManager;
use tauri::{window::{Effect, EffectState, EffectsBuilder}, Manager, Window};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};

mod global_shortcuts;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let mut global_shortcuts = GlobalShortcuts::new();
    global_shortcuts.add_shortcut(
        Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN), 
        |window| switch_window_visibility(window));

    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("main").unwrap();
            apply_window_effects(&window);

            window.on_window_event(|event| {
                println!("Window event: {:?}", event);
                if let tauri::WindowEvent::Focused(focused) = event {
                    println!("Window focused: {}", focused);
                }
            });

            let window = Arc::new(window);
            global_shortcuts.register(&app, window)?;

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    unsafe {
        load_plugin();
    }
}

unsafe fn load_plugin() {
    const PATH: &str = "C:\\Users\\bubbl\\Documents\\sumitapp\\src\\local\\plugins\\finder\\dist";
    const FILENAME: &str = "finder.dll";

    info!("Try to load plugin from {:?}", PATH);

    let path = std::path::Path::new(PATH).join(FILENAME);
    let filename = path.to_str().unwrap();

    let mut plugin_manager = PluginManager::new();
    
    if let Ok(_) = plugin_manager.load_plugin(filename) {
        info!("Plugin loaded successfully");
    } else {
        error!("Plugin failed to load");
    }
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

fn apply_window_effects(window: &Window) {
    window.set_effects(
        EffectsBuilder::new()
        .state(EffectState::Active)
        .effect(Effect::Acrylic)
        .build()
    ).unwrap();
}