pub mod instancing;

use self_update as su;
use su::cargo_crate_version;
use libloading as ll;
use notify;
use std::{path::{Path, PathBuf}, time};

use fuse_util as util;
use fuse_gui as fgui;

fn main() {
    let _instances = instancing::InstanceMap::try_new("fuse");
        let _ = match _instances {
            Ok(_instances) => _instances,
            Err(e) => panic!("Process already running?: {}", e)
        };
    
    /*
    if let Err(e) = check_update() {
        println!("[ERROR] {}", e);
        ::std::process::exit(1);
    }
    */

    scoped_main();        
}

fn scoped_main() {
    let mut ui = fgui::FGui::new("Fuse".to_string());
    //while !ui.b_request_exit {
        ui.run();
    //}
}

fn check_update() -> Result<(), Box<dyn ::std::error::Error>> {
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("BarronKane")
        .repo_name("Fuse")
        .build()?
        .fetch()?;
    println!("found releases:");
    println!("{:#?}\n", releases);

    let status = self_update::backends::github::Update::configure()
        .repo_owner("BarronKane")
        .repo_name("Fuse")
        .bin_name("Fuse")
        .show_download_progress(true)
        //.target_version_tag("v9.9.10")
        //.show_output(false)
        //.no_confirm(true)
        //
        // For private repos, you will need to provide a GitHub auth token
        // **Make sure not to bake the token into your app**; it is recommended
        // you obtain it via another mechanism, such as environment variables
        // or prompting the user for input
        //.auth_token(env!("DOWNLOAD_AUTH_TOKEN"))
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}
