use core::panic;
use notify::{
    event::{DataChange, ModifyKind, RenameMode},
    EventKind, Watcher,
};
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};

pub const EXTENSION_DIRECTORY: &str = r"sumit-app\src\plugins";

pub struct ExtensionWatcher {
    pub watcher: notify::RecommendedWatcher,
}

impl ExtensionWatcher {
    pub fn new(handle: &AppHandle) -> notify::Result<Self> {
        let watch_dir = Self::get_extension_full_dir();
        let handle_clone = handle.clone();

        let mut watcher =
            notify::recommended_watcher(move |event| Self::event_handler(event, &handle_clone))?;

        // Start to watch the extension directory
        Self::watch(&mut watcher, &watch_dir)?;
        Ok(Self { watcher })
    }

    pub fn watch(watcher: &mut notify::RecommendedWatcher, path: &str) -> notify::Result<()> {
        watcher
            .watch(Path::new(path), notify::RecursiveMode::Recursive)
            .expect(format!("Failed to watch path: {:?}", path).as_str());
        Ok(())
    }

    #[allow(dead_code)]
    pub fn unwatch(&mut self, path: &str) -> notify::Result<()> {
        self.watcher.unwatch(Path::new(path))
    }

    fn event_handler(event: notify::Result<notify::Event>, app: &AppHandle) {
        match event {
            Ok(event) => {
                match event.kind {
                    EventKind::Any => {}
                    EventKind::Access(_) => {}
                    EventKind::Create(_) => {}
                    EventKind::Remove(_) => {}
                    EventKind::Modify(kind) => Self::on_modify_entry(&app, &event, &kind),
                    EventKind::Other => {}
                }

                println!("{:?}", event)
            }
            Err(e) => {
                println!("{:?}", e)
            }
        }
    }

    fn on_modify_entry(app: &AppHandle, event: &notify::Event, modify_kind: &ModifyKind) {
        match modify_kind {
            ModifyKind::Any => {
                for path in &event.paths {
                    println!("File changed: {:?}", &path);
                    app.emit("ON_FILE_CHANGED", &path.display().to_string())
                        .unwrap();
                }
            }
            ModifyKind::Name(rename) => match rename {
                RenameMode::Any | RenameMode::To => {
                    for path in &event.paths {
                        // if path.extension() == Some(OsStr::new("dll")) {
                        //     println!("File changed: {:?}", path);
                        //     app.emit("ON_FILE_CHANGED", path.display().to_string()).unwrap();
                        // }

                        println!("File renamed: {:?}", path);
                        app.emit("ON_FILE_RENAMED", path.display().to_string())
                            .unwrap();
                    }
                }
                _ => {}
            },
            ModifyKind::Data(_) => {}
            ModifyKind::Metadata(_) => {}
            ModifyKind::Other => {}
        }
    }

    fn get_extension_full_dir() -> String {
        if let Some(documents_dir) = dirs::document_dir() {
            let full_path = PathBuf::from(documents_dir).join(EXTENSION_DIRECTORY);
            
            println!("Extension full path: {:?}", full_path);

            if let Some(extension_path) = full_path.to_str() {
                extension_path.to_string()
            } else {
                panic!("Failed to convert extension path to string");
            }
        } else {
            panic!("Failed to get documents directory");
        }
    }
}
