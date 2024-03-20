#[macro_use]
extern crate log;

#[macro_use]
extern crate plugit;

use std::str;
use plugit::Plugin;

#[derive(Debug, Default)]
pub struct FinderPlugin;

impl Plugin for FinderPlugin {
    fn name(&self) -> &'static str {
        "Finder"
    }

    fn on_plugin_load(&self) {
        info!("Finder plugin loaded");
    }

    fn on_plugin_unload(&self) {
        info!("Finder plugin unloaded");
    }
}

declare_plugin!(FinderPlugin, FinderPlugin::default);