use core::panic;
use notify::{
    event::{DataChange, ModifyKind, RenameMode},
    EventKind, Watcher,
};
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use tauri::{AppHandle, Manager};

use crate::{
    dotnet::{compile_csharp_assembly, ReleaseMode}, paths, watcher::playload::{AssemblyUpdatedPayload, ChangedPayload, RenamedPayload}
};

pub mod playload;

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

                    app.emit(
                        "FileChanged",
                        ChangedPayload {
                            path: path.display().to_string(),
                            is_dir: path.is_dir(),
                        },
                    )
                    .unwrap();
                    
                    let watch_dir = Self::get_extension_full_dir();

                    let allowed_extensions = ["cs", "cshtml", "razor", "html", "js", "css", "ts", "scss", "less", "json", "csproj"];

                    if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                        if allowed_extensions.contains(&extension) {
                            let path_str = path.to_str().unwrap();
                            let root_dir = path_str.replace(watch_dir.as_str(), "");

                            println!("New path: {:?}", root_dir);
        
                            if let Ok(is_compiled) = compile_csharp_assembly(path_str, ReleaseMode::Release)
                            {
                                if is_compiled {
                                    let mut file = File::open(path).unwrap();
                                    let mut buffer = Vec::new();
                                    file.read_to_end(&mut buffer).unwrap();
        
                                    let buffer = Box::leak(buffer.into_boxed_slice());
        
                                    app.emit(
                                        "AssemblyUpdated",
                                        AssemblyUpdatedPayload {
                                            path: path.display().to_string(),
                                            is_dir: path.is_dir(),
                                            buffer,
                                        },
                                    )
                                    .unwrap();
        
                                    println!("Assembly updated: {:?}", &path);
                                } else {
                                    panic!("Failed to compile C# assembly");
                                }
                            } else {
                            }
                        }
                    }
                }
            }
            ModifyKind::Name(rename) => match rename {
                RenameMode::Any | RenameMode::To => {
                    for path in &event.paths {
                        println!("File renamed: {:?}", path);

                        app.emit(
                            "FileRenamed",
                            RenamedPayload {
                                path: path.display().to_string(),
                                is_dir: path.is_dir(),
                            },
                        )
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
