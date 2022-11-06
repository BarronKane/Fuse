mod app;
pub use app::FApp;

use eframe::egui;

pub struct FGui {
    Title: String,
}

impl eframe::App for FGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

    }
}

impl FGui {
    pub fn new(Title: String) -> FGui {
        
        
        FGui {
            Title
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn run(&mut self) -> &Self {
        let native_options = eframe::NativeOptions::default();
        
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
