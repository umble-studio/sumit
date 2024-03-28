use std::any::Any;

use anyhow::anyhow;
use libloading::Library;
use tauri::{AppHandle, Manager};

use super::{manifest::ExtensionManifest, registry::ExtensionRegistry};

#[macro_export]
macro_rules! declare_extension {
    ($extension_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _extension_create() -> *mut dyn $crate::Extension {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $extension_type = $constructor;

            let object = constructor();
            let boxed: Box<dyn $crate::Extension> = Box::new(object);

            Box::into_raw(boxed)
        }
    };
}

#[derive(Debug)]
pub struct Extension {
    pub manifest: ExtensionManifest,
    pub enabled: bool,
}

pub trait IExtension: Any + Send + Sync {
    fn state(&self) -> ExtensionState;
    fn manifest(&self) -> ExtensionManifest;
    fn on_load(&self);
    fn on_unload(&self);
}

impl Extension {
    pub fn new(&self, manifest: ExtensionManifest) -> Self {
        Self {
            manifest,
            enabled: false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExtensionState {
    Enabled,
    Disabled,
}

impl IExtension for Extension {
    fn state(&self) -> ExtensionState {
        if self.enabled {
            ExtensionState::Enabled
        } else {
            ExtensionState::Disabled
        }
    }

    fn manifest(&self) -> ExtensionManifest {
        self.manifest.clone()
    }

    fn on_load(&self) {}
    fn on_unload(&self) {}
}

#[derive(Debug)]
pub struct ExtensionManager<'a> {
    pub handle: &'a AppHandle,
    pub registry: ExtensionRegistry,
    pub extensions: Vec<Extension>,
    pub loaded_extensions: Vec<Library>,
}

impl<'a> ExtensionManager<'a> {
    pub fn new(handle: &'a AppHandle) -> Self {
        let registry = ExtensionRegistry::default();

        Self {
            handle,
            registry,
            extensions: vec![],
            loaded_extensions: vec![],
        }
    }

    pub fn load_extensions(&mut self) {
        self.registry.load_manifests();

        for manifest_info in self.registry.get_manifests() {
            let manifest_path = manifest_info.0;
            let manifest = manifest_info.1;

            if let Some(extension) = self.load_extension(&manifest_path, manifest.clone()) {
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
    ) -> Option<Extension> {
        todo!()
    }

    /**
     * Unload a server side extension
     */
    pub fn unload_extension(&mut self, name: &str) {
        todo!()
    }

    /**
     * Enable an extension on client & server side
     */
    pub fn enable_extension(&mut self, name: &str) -> Result<(), tauri::Error> {
        if let Some(extension) = self.get_extension(name) {
            extension.enabled = true;
            self.handle.emit("extension:enabled", ())?;

            Ok(())
        } else {
            Err(tauri::Error::Anyhow(anyhow!(
                "Failed to enable extension: {}",
                name
            )))
        }
    }

    /**
     * Disable an extension on client & server side
     */
    pub fn disable_extension(&mut self, name: &str) -> Result<(), tauri::Error> {
        if let Some(extension) = self.get_extension(name) {
            extension.enabled = false;
            self.handle.emit("extension:disabled", ())?;

            Ok(())
        } else {
            Err(tauri::Error::Anyhow(anyhow!(
                "Failed to disable extension: {}",
                name
            )))
        }
    }

    pub fn get_extension(&mut self, name: &str) -> Option<&mut Extension> {
        if let Some(extension) = self.extensions.iter_mut().find(|e| e.manifest.name == name) {
            Some(extension)
        } else {
            None
        }
    }
}