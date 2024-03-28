use std::any::Any;

use libloading::Library;
use tauri::{AppHandle, Manager};

use super::manifest::Manifest;

pub struct ExtensionHandle<'a> {
    pub(crate) handle: Option<&'a AppHandle>,
    pub(crate) manifest: Manifest,
    pub(crate) enabled: bool,
    pub(crate) instance: Option<Box<dyn Extension>>,
    pub(crate) lib: Option<Library>,
}

impl<'a> ExtensionHandle<'a> {
    pub fn new(manifest: Manifest, handle: Option<&'a AppHandle>) -> Self {
        Self {
            handle,
            manifest,
            enabled: false,
            instance: None,
            lib: None,
        }
    }
}

pub trait Extension: Any + Send + Sync {
    fn on_load(&mut self, extension: &mut ExtensionHandle);
    fn on_unload(&mut self);
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum State {
    Enabled,
    Disabled,
}

pub trait ExtensionState {
    fn enable(&mut self);
    fn disable(&mut self);
    fn state (&self) -> State;
}

impl ExtensionState for ExtensionHandle<'_> {
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

    fn state (&self) -> State {
        if self.enabled {
            State::Enabled
        } else {
            State::Disabled
        }
    }
}