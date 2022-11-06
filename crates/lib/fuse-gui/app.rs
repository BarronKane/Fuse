use crate::filedialog;

use std::{path::PathBuf};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct FApp {
    Title: String,

    #[serde(skip)]
    value: f32,

    #[serde(skip)]
    path_dialog: filedialog::ImNativeFileDialog<Option<PathBuf>>,

    input_path: PathBuf,
    output_path: PathBuf

    
}

impl Default for FApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            Title: "Hello World!".to_owned(),
            value: 2.7,

            path_dialog: filedialog::ImNativeFileDialog::default(),

            input_path: PathBuf::default(),
            output_path: PathBuf::default(),
        }
    }
}

impl FApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for FApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { 
            Title, 
            value ,
            path_dialog,
            input_path,
            output_path
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        if let Some(result) = self.path_dialog.check() {
            match result {
                Ok(Some(path)) => self.input_path = path,
                Ok(None) => {}
                Err(error) => {}
            }
        }

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(Title);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Input Directory:");
            
            // File Dialog
            ui.label("Path");
            let text_original = self.input_path.to_string_lossy().to_string();
            let mut text_edit = text_original.clone();
            ui.text_edit_singleline(&mut text_edit);
            if text_edit != text_original {
                self.input_path = PathBuf::from(text_edit);
            }
            if ui.button("Browse").clicked() {
                let location = self.input_path.clone();
                //let repaint_signal = frame.;
                self.path_dialog
                   .open_single_dir(Some(location))
                   .expect("Unable to open file_path_dialog");
            }


            ui.hyperlink("https://github.com/BarronKane/Fuse");
            ui.add(egui::github_link_file!(
                "https://github.com/BarronKane/Fuse/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
