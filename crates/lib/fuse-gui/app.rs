use crate::ffmpeg;
use crate::ffmpeg::DownloadInfo;
use crate::ffmpeg::FfmpegInfo;
use crate::filedialog;

use flume::Receiver;
use std::path::PathBuf;

pub struct ChannelsForGuiThread {
    //download_info_rx: Receiver<ffmpeg::DownloadInfo>
}

pub struct ChannelsForFfmpegThread {
    //pub download_info_tx: Sender<ffmpeg::DownloadInfo>
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct FApp {
    title: String,

    ffmpeg_info: FfmpegInfo,

    #[serde(skip)]
    channels_for_gui: ChannelsForGuiThread,

    #[serde(skip)]
    value: f32,

    #[serde(skip)]
    path_dialog: filedialog::ImNativeFileDialog<Option<PathBuf>>,
    #[serde(skip)]
    path_dialog_out: filedialog::ImNativeFileDialog<Option<PathBuf>>,

    input_path: PathBuf,
    output_path: PathBuf,

    ffmpeg_is_ready: bool,

    #[serde(skip)]
    download_info: DownloadInfo,
    #[serde(skip)]
    download_info_rx: Option<Receiver<DownloadInfo>>,

    scalar: f32,
    total: f32,
}

impl Default for FApp {
    fn default() -> Self {
        let (download_info_tx, download_info_rx) = flume::bounded(1);

        let channels_for_gui = ChannelsForGuiThread {};

        let channels_for_ffmpeg = ChannelsForFfmpegThread {};

        let mut ffmpeg_info = ffmpeg::FfmpegInfo::default();
        ffmpeg_info.channels_for_ffmpeg = Some(channels_for_ffmpeg);
        ffmpeg_info.download_info_tx = Some(download_info_tx);

        Self {
            // Example stuff:
            title: "Fuse".to_owned(),

            ffmpeg_info,
            channels_for_gui,

            value: 2.7,

            path_dialog: filedialog::ImNativeFileDialog::default(),
            path_dialog_out: filedialog::ImNativeFileDialog::default(),

            input_path: PathBuf::default(),
            output_path: PathBuf::default(),

            ffmpeg_is_ready: false,
            download_info: DownloadInfo::default(),
            download_info_rx: Some(download_info_rx),

            scalar: 0.0,
            total: 0.0,
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
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            title,
            ffmpeg_info,
            channels_for_gui: channels_for_ffmpeg,
            value,
            path_dialog,
            path_dialog_out,
            input_path,
            output_path,
            ffmpeg_is_ready,
            download_info,
            download_info_rx,
            scalar,
            total,
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

        if let Some(result) = self.path_dialog_out.check() {
            match result {
                Ok(Some(path)) => self.output_path = path,
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
            ui.heading("FFMPEG");

            if ui.button("Download FFMPEG").clicked() {
                ffmpeg_info.download_ffmpeg();
                self.ffmpeg_is_ready = true;
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
            let text_original_in = self.input_path.to_string_lossy().to_string();
            let mut text_edit_in = text_original_in.clone();
            ui.text_edit_singleline(&mut text_edit_in);
            if text_edit_in != text_original_in {
                self.input_path = PathBuf::from(text_edit_in);
            }
            if ui.button("Browse").clicked() {
                let location = self.input_path.clone();
                //let repaint_signal = frame.;
                self.path_dialog
                    .open_single_dir(Some(location))
                    .expect("Unable to open file_path_dialog");
            }

            ui.heading("OutputDirectory:");
            //File Dialog
            let text_original_out = self.output_path.to_string_lossy().to_string();
            let mut text_edit_out = text_original_out.clone();
            ui.text_edit_singleline(&mut text_edit_out);
            if text_edit_out != text_original_out {
                self.output_path = PathBuf::from(text_edit_out);
            }
            if ui.button("Browse").clicked() {
                let location = self.output_path.clone();
                //let repaint_signal = frame.;
                self.path_dialog_out
                    .open_single_dir(Some(location))
                    .expect("Unable to open file_path_dialog");
            }
        });

        egui::TopBottomPanel::bottom("footer_panel").show(ctx, |ui| {
            ui.hyperlink("https://github.com/BarronKane/Fuse");
            ui.add(egui::github_link_file!(
                "https://github.com/BarronKane/Fuse/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });

        egui::TopBottomPanel::bottom("status_panel").show(ctx, |ui| {
            let download_info = self.download_info_rx.as_ref().unwrap();
            if !download_info.is_empty() {
                let _download_info = download_info.recv().unwrap();
                self.total = _download_info.content_length as f32;
                self.scalar = _download_info.downloaded as f32;
            }
            let progress = self.scalar / self.total;
            let progress_bar = egui::ProgressBar::new(progress)
                .show_percentage()
                .animate(true);
            ui.add(progress_bar);
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
