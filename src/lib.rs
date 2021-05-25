#[macro_use]
extern crate vst;

use vst::plugin::{HostCallback, Info, Plugin};

struct BasicPlugin;

impl Plugin for BasicPlugin {
    fn new(_host: HostCallback) -> Self {
        BasicPlugin
    }

    fn get_info(&self) -> Info {
        Info {
            name: "Basic Plugin".to_string(),
            unique_id: 1357, // Used by hosts to differentiate between plugins.
            ..Default::default()
        }
    }
}

plugin_main!(BasicPlugin); // Important!