pub mod instancing;

use libloading as ll;
use notify;
use std::{path::{Path, PathBuf}, time};

use fuse_util as util;
use fuse_gui as fgui;

fn main() {
    let _instances = instancing::InstanceMap::try_new("fuse");
        let _ = match _instances {
            Ok(_instances) => _instances,
            Err(e) => panic!("Process already running: {}", e)
        };

    scoped_main();        
}

fn scoped_main() {
    let mut ui = fgui::FGui::new("Fuse".to_string());
    ui.run();
}
