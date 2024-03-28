use std::{
    any::Any,
    borrow::{Borrow, BorrowMut},
    path::Path,
};

use anyhow::anyhow;
use libloading::{Library, Symbol};
use tauri::{utils::acl::manifest, AppHandle, Manager};

use crate::extension;

use super::{manifest::ExtensionManifest, registry::ExtensionRegistry};

#[macro_export]
macro_rules! declare_extension {
    ($extension_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _extension_create() -> *mut dyn $crate::extension::manager::IExtension {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $extension_type = $constructor;

            let object = constructor();
            let boxed: Box<dyn $crate::extension::manager::IExtension> = Box::new(object);

            Box::into_raw(boxed)
        }
    };
}

pub struct Extension<'a> {
    pub(crate) handle: Option<&'a AppHandle>,
    pub(crate) manifest: ExtensionManifest,
    pub(crate) enabled: bool,
    pub(crate) instance: Option<Box<dyn IExtension>>,
    pub(crate) lib: Option<Library>,
}

pub trait IExtension: Any + Send + Sync {
    fn on_load(&mut self, extension: &mut Extension);
    fn on_unload(&mut self);
}

impl<'a> Extension<'a> {
    pub fn new(manifest: ExtensionManifest, handle: Option<&'a AppHandle>) -> Self {
        Self {
            handle,
            manifest,
            enabled: false,
            instance: None,
            lib: None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExtensionState {
    Enabled,
    Disabled,
}

pub trait IExtensionState {
    fn enable(&mut self);
    fn disable(&mut self);
    fn state (&self) -> ExtensionState;
}

impl<'a> IExtensionState for Extension<'a> {
    fn enable(&mut self) {
        self.enabled = true;

        if let Some(handle) = self.handle {
            handle.emit("extension:enabled", ()).unwrap();
            println!("Enable extension: {}", self.manifest.name);
        } else {
            println!("Failed to emit enable extension: {}", self.manifest.name);
        }
    }

    fn disable(&mut self) {
        self.enabled = false;
   
        if let Some(handle) = self.handle {
            handle.emit("extension:disabled", ()).unwrap();
            println!("Disable extension: {}", self.manifest.name);
        } else {
            println!("Failed to emit disable extension: {}", self.manifest.name);
        }
    }

    fn state (&self) -> ExtensionState {
        if self.enabled {
            ExtensionState::Enabled
        } else {
            ExtensionState::Disabled
        }
    }
}

pub(crate) struct ExtensionManager<'a> {
    pub handle: &'a AppHandle,
    pub registry: ExtensionRegistry,
    pub extensions: Vec<Extension<'a>>,
}

impl<'a> ExtensionManager<'a> {
    pub fn new(handle: &'a AppHandle) -> Self {
        let registry = ExtensionRegistry::default();

        Self {
            handle,
            registry,
            extensions: vec![],
        }
    }

    pub fn load_extensions(&mut self) {
        self.registry.load_manifests();

        for manifest_info in self.registry.get_manifests() {
            let manifest_path = manifest_info.0;
            let manifest = manifest_info.1;

            if let Ok(_) = self.load_extension(&manifest_path, manifest.clone()) {
                println!("Loaded extension: {}", manifest.name);
            } else {
                println!("Failed to load extension: {}", manifest.name);
            }
        }
    }

    /**
     * Load a server side extension
     */
    pub fn load_extension(
        &mut self,
        manifest_path: &str,
        manifest: ExtensionManifest,
    ) -> Result<(), tauri::Error> {
        type ExtensionCreate = extern "C" fn() -> *mut dyn IExtension;
        const EXTENSION_SYMBOL: &'static [u8] = b"_extension_create";

        let manifest_clone = manifest.clone();

        if let Some(server_info) = manifest_clone.server {
            let filename = Path::new(manifest_path)
                .parent()
                .unwrap()
                .join(server_info.entrypoint);

            println!(
                "Loading extension: {} at path: {}",
                manifest.name,
                filename.to_str().unwrap().to_string()
            );

            unsafe {
                if let Ok(lib) = Library::new(filename) {
                    let manifest_clone = manifest.clone();
                    let mut extension = Extension::new(manifest_clone, Some(self.handle));

                    let constructor: Symbol<ExtensionCreate> =
                        lib.get(EXTENSION_SYMBOL).expect(&format!(
                            "The `{}` symbol wasn't found.",
                            String::from_utf8_lossy(EXTENSION_SYMBOL).to_string()
                        ));

                    let boxed_raw = constructor();

                    self.enable_extension(&mut extension)?;
                    
                    let mut instance = Box::from_raw(boxed_raw);
                    instance.on_load(&mut extension);
                    
                    self.extensions.push(extension);

                    // println!("Loaded extension: {}", manifest_clone.name);
                    Ok(())
                } else {
                    println!(
                        "Failed to find library {} at path: {}",
                        manifest.name, manifest_path
                    );
                    Err(tauri::Error::Anyhow(anyhow!(
                        "Failed to find library {} at path: {}",
                        manifest.name,
                        manifest_path
                    )))
                }
            }
        } else {
            Ok(())
        }
    }

    /**
     * Unload a server side extension
     */
    pub fn unload_extension(&mut self, extension: &mut Extension) {
        self.disable_extension(extension).unwrap();
        self.extensions.retain(|e| e.manifest.name != extension.manifest.name);

        // extension.on_unload();
    }

    /**
     * Enable an extension on client & server side
     */
    pub fn enable_extension(&mut self, extension: &mut Extension) -> Result<(), tauri::Error> {
        extension.enabled = true;
        self.handle.emit("extension:enabled", ())?;

        Ok(())
    }

    /**
     * Disable an extension on client & server side
     */
    pub fn disable_extension(&mut self, extension: &mut Extension) -> Result<(), tauri::Error> {
        extension.enabled = false;
        self.handle.emit("extension:disabled", ())?;

        Ok(())
    }

    pub fn get_extension(&mut self, name: &str) -> Option<&mut Extension<'a>> {
        if let Some(extension) = self.extensions.iter_mut().find(|e| e.manifest.name == name) {
            Some(extension)
        } else {
            None
        }
    }

    pub fn get_extension_lib(&mut self, name: &str) -> Option<&Library> {
        if let Some(extension) = self.extensions.iter().find(|e| e.manifest.name == name) {
            if let Some(lib) = extension.lib.as_ref() {
                Some(lib)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use dirs::document_dir;
    use tauri::utils::acl::manifest;

    use crate::extension::constant::EXTENSION_DIRECTORY;

    use super::*;

    // #[test]
    // fn test_load_extension() {
    //     // const MANIFEST: &str =
    //     //     r"C:\Users\bubbl\Documents\sumit-app\src\plugins\Finder\server\dist\server.dll";

    //     unsafe {
    //         let document_dir = document_dir().unwrap();
    //         let extension_dir = Path::new(document_dir.to_str().unwrap()).join(EXTENSION_DIRECTORY);
    //         let manifest_path = Path::new(extension_dir.to_str().unwrap())
    //             .join("Finder")
    //             .join("server")
    //             .join("dist")
    //             .join("server.dll");

    //         println!("Manifest path: {}", manifest_path.to_str().unwrap());

    //         type ExtensionCreate = extern "C" fn() -> *mut dyn IExtension;
    //         const EXTENSION_SYMBOL: &'static [u8] = b"_extension_create";

    //         if let Ok(lib) = Library::new(manifest_path) {
    //             let constructor: Symbol<ExtensionCreate> =
    //                 lib.get(EXTENSION_SYMBOL).expect(&format!(
    //                     "The `{}` symbol wasn't found.",
    //                     String::from_utf8_lossy(EXTENSION_SYMBOL).to_string()
    //                 ));

    //             let boxed_raw = constructor();

    //             let manifest = ExtensionManifest::default();
    //             let mut extension = Extension::new(manifest);

    //             let mut instance = Box::from_raw(boxed_raw);
    //             instance.on_load(&mut extension);
                
    //             assert!(true);
    //         } else {
    //             assert!(false);
    //         }
    //     }
    // }
}
