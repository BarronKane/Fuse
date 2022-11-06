mod app;
pub use app::FApp;
pub mod filedialog;

use fuse_util as util;

use eframe::egui;
use std::path::PathBuf;

#[derive(Default)]
pub struct FGui {
    Title: String,
}

impl eframe::App for FGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

    }
}

impl FGui {
    pub fn new(Title: String) -> Self {
        
        let mut fgui = FGui::default();
        fgui.Title = Title;

        fgui
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn run(&mut self) -> &Self {
        let mut native_options = eframe::NativeOptions::default();
        let icon = util::file_to_bytes(util::get_cwd().unwrap().join("Assets").join("icon-1024.png"));
        native_options.icon_data = Some(eframe::IconData {
            rgba: icon.to_vec(),
            width: 32,
            height: 32,
        });
        
        eframe::run_native(&self.Title, 
            native_options, 
            Box::new(|cc| Box::new(FApp::new(cc))),
        );
        
        self
    }

    #[cfg(target_arch = "wasm32")]
    pub fn run(&mut self) -> &Self {
        let web_options = eframe::WebOptions::default();
        /*
        eframe::start_web(
            
        )
        .expect("Failed to start eframe!");
        */

        self
    }
}

fn ui_counter(ui: &mut egui::Ui, counter: &mut i32) {
    // Put the buttons and label on the same row:
    ui.horizontal(|ui| {
        if ui.button("-").clicked() {
            *counter -= 1;
        }
        ui.label(counter.to_string());
        if ui.button("+").clicked() {
            *counter += 1;
        }
    });
}
