use super::registry::{ExtensionManifest, ExtensionRegistry};

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

#[derive(Debug, Default)]
pub struct ExtensionManager {
    pub registry: ExtensionRegistry,
    pub extensions: Vec<Extension>,
}

impl ExtensionManager {
    pub fn load_extensions(&mut self) {
        todo!()
    }

    pub fn load_extension(&mut self, path: &str, manifest: ExtensionManifest) {
        todo!()
    }

    pub fn unload_extension(&mut self, name: &str) {
        todo!()
    }

    pub fn enable_extension(&mut self, name: &str) -> Result<(), ()> {
        if let Some(extension) = self.get_extension(name) {
            extension.enabled = true;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn disable_extension(&mut self, name: &str) -> Result<(), ()> {
        if let Some(extension) = self.get_extension(name) {
            extension.enabled = false;
            Ok(())
        } else {
            Err(())
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
