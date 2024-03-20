use std::error::Error;
use std::{collections::HashMap, sync::Arc};
use tauri::{App, Window};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

pub struct GlobalShortcuts {
    shortcuts: HashMap<Shortcut, Arc<dyn Fn(&Window) -> Result<(), Box<dyn Error>> + Send + Sync>>,
}

impl GlobalShortcuts {
    pub fn new() -> Self {
        GlobalShortcuts {
            shortcuts: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        app: &App,
        window: Arc<Window>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (shortcut, action) in &self.shortcuts {
            self.register_shortcut_handler(app, &shortcut, action.clone(), window.clone())?;
            app.global_shortcut().register(shortcut.clone())?;
        }
        Ok(())
    }

    fn register_shortcut_handler(
        &self,
        app: &App,
        shortcut: &Shortcut,
        action: Arc<dyn Fn(&Window) -> Result<(), Box<dyn Error>> + Send + Sync>,
        window: Arc<Window>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let shortcut = shortcut.clone();

        app.handle()
            .plugin(
                tauri_plugin_global_shortcut::Builder::with_handler(
                    move |_app, received_shortcut| {
                        if received_shortcut == &shortcut {
                            if let Err(err) = action(&window) {
                                eprintln!("Error executing shortcut action: {}", err);
                            }
                        }
                    },
                )
                .build(),
            )
            .unwrap();
        Ok(())
    }

    pub fn add_shortcut<F>(&mut self, shortcut: Shortcut, action: F)
    where
        F: Fn(&Window) -> Result<(), Box<dyn Error>> + 'static + Send + Sync,
    {
        self.shortcuts.insert(shortcut, Arc::new(action));
    }

    #[allow(dead_code)]
    pub fn remove_shortcut(&mut self, shortcut: Shortcut) {
        self.shortcuts.remove(&shortcut);
    }
}
