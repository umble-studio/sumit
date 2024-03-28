use anyhow::anyhow;
use tauri::{AppHandle, Manager};

use super::{manifest::ExtensionManifest, registry::ExtensionRegistry};

#[derive(Debug)]
pub struct Extension {
    pub manifest: ExtensionManifest,
    pub enabled: bool,
}

impl Extension {
    pub fn new(&self, manifest: ExtensionManifest) -> Self {
        Self {
            manifest,
            enabled: false,
        }
    }
}

#[derive(Debug)]
pub struct ExtensionManager<'a> {
    pub handle: &'a AppHandle,
    pub registry: ExtensionRegistry,
    pub extensions: Vec<Extension>,
}

impl<'a> ExtensionManager<'a> {
    pub fn new(handle: &'a AppHandle) -> Self {
        let mut registry = ExtensionRegistry::default();
        registry.load_manifests();

        Self {
            handle,
            registry,
            extensions: vec![],
        }
    }

    pub fn load_extensions(&mut self) {
        for manifest in self.registry.get_manifests() {
            if let Some(extension) = self.load_extension(todo!(), manifest.clone()) {
                println!("Loaded extension: {}", manifest.name);
            } else {
                println!("Failed to load extension: {}", manifest.name);
            }
        }
    }

    /**
     * Load a server side extension
     */
    pub fn load_extension(&mut self, manifest_path: &str, manifest: ExtensionManifest) -> Option<Extension> {
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
            Err(tauri::Error::Anyhow(anyhow!("Failed to enable extension: {}", name)))
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
            Err(tauri::Error::Anyhow(anyhow!("Failed to disable extension: {}", name)))
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
