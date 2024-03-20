extern crate libloading;

use anyhow::Result;
use libloading::{Library, Symbol};
use std::any::Any;

#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut dyn $crate::Plugin {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $plugin_type = $constructor;

            let object = constructor();
            let boxed: Box<dyn $crate::Plugin> = Box::new(object);

            Box::into_raw(boxed)
        }
    };
}

pub trait Plugin: Any + Send + Sync {
    fn name(&self) -> &'static str;
    fn on_plugin_load(&self) {}
    fn on_plugin_unload(&self) {}
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    loaded_libraries: Vec<Library>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            loaded_libraries: Vec::new(),
        }
    }

    pub unsafe fn load_plugin(&mut self, filename: &str) -> Result<()> {
        type PluginCreate = extern fn() -> *mut dyn Plugin;

        let lib = Library::new(filename).expect("Failed to load plugin library");
        self.loaded_libraries.push(lib);

        let lib = self.loaded_libraries.last().unwrap();

        let constructor: Symbol<PluginCreate> = lib
        .get(b"_plugin_create")
        .expect("The `_plugin_create` symbol wasn't found.");

        let boxed_raw = constructor();
        let plugin = Box::from_raw(boxed_raw);

        plugin.on_plugin_load();
        self.plugins.push(plugin);

        Ok(())
    }

    pub fn unload(&mut self) {
        for plugin in self.plugins.drain(..) {
            plugin.on_plugin_unload();
        }

        for lib in self.loaded_libraries.drain(..) {
            drop(lib);
        }
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        if !self.plugins.is_empty() || !self.loaded_libraries.is_empty() {
            self.unload();
        }
    }
}
