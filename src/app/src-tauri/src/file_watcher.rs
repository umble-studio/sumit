use std::path::Path;
use notify::{
    event::{DataChange, ModifyKind, RenameMode},
    EventKind, Watcher,
};
use tauri::{AppHandle, Manager};

pub const EXTENSION_DIRECTORY: &str = r"C:\Users\bubbl\Documents\sumit-app\src\plugins";

pub struct ExtensionWatcher {
    pub watcher: notify::RecommendedWatcher,
}

impl ExtensionWatcher {
    pub fn new(handle: &AppHandle) -> notify::Result<Self> {
        let handle_clone = handle.clone();

        let mut watcher =
            notify::recommended_watcher(move |event| Self::event_handler(event, &handle_clone))?;

        // Start to watch the extension directory
        Self::watch(&mut watcher, EXTENSION_DIRECTORY)?;
        Ok(Self { watcher })
    }

    pub fn watch(watcher: &mut notify::RecommendedWatcher, path: &str) -> notify::Result<()> {
        // Commence à surveiller le répertoire récursivement
        watcher
            .watch(Path::new(path), notify::RecursiveMode::Recursive)
            .expect("Failed to watch path");
        Ok(())
    }

    #[allow(dead_code)]
    pub fn unwatch(&mut self, path: &str) -> notify::Result<()> {
        // Arrête de surveiller le répertoire
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
                    EventKind::Modify(kind) => Self::on_modify_entry(&app, &event,&kind),
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
                    app.emit("ON_FILE_CHANGED", &path.display().to_string()).unwrap();
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
            ModifyKind::Metadata(_) => {},
            ModifyKind::Other => {},
        }
    }
}
