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
    fn register_commands(&self) {}
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
        type PluginCreate = extern "C" fn() -> *mut dyn Plugin;
        const PLUGIN_SYMBOL: &'static [u8] = b"_plugin_create";

        let lib = Library::new(filename).expect("Failed to load plugin library");
        self.loaded_libraries.push(lib);

        let lib = self.loaded_libraries.last().unwrap();

        let constructor: Symbol<PluginCreate> = lib.get(PLUGIN_SYMBOL).expect(&format!(
            "The `{}` symbol wasn't found.",
            String::from_utf8_lossy(PLUGIN_SYMBOL).to_string()
        ));

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

#[cfg(test)]
mod tests {
    use crate::PluginManager;
    use libloading::Library;

    const DLL_PATH: &str = "../../src/local/plugins/finder/dist/finder.dll";

    #[test]
    fn test_load_library_from_relative_path() {
        unsafe {
            let lib = Library::new(DLL_PATH);
            assert_eq!(lib.is_ok(), true);
        }
    }

    #[test]
    fn test_load_library_from_plugin_manager() {
        unsafe {
            let mut plugin_manager = PluginManager::new();

            if let Ok(_) = plugin_manager.load_plugin(DLL_PATH) {
                assert!(true);
            } else {
                assert!(false);
            }
        }
    }
}
