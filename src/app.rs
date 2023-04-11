use std::path::PathBuf;

use crate::PlinitImage;

#[derive(Debug)]
pub struct ViewState {
    pub zoom: f32,
    pub offset: egui::Vec2,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            zoom: 1.,
            offset: egui::Vec2::ZERO,
        }
    }
}

#[derive(Default)]
pub struct PlinitApp {
    images: Vec<PlinitImage>,
    view_state: ViewState,
}

impl PlinitApp {
    pub fn load_image(&mut self, ctx: &egui::Context, path: &Option<PathBuf>) {
        if let Some(path) = path {
            let path = path.to_str().unwrap_or("");
            match PlinitImage::load(ctx, path) {
                Ok(image) => self.images.push(image),
                Err(err) => log::error!("Failed to load {} - {}", path, err),
            }
        }
    }
}

impl eframe::App for PlinitApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::panel::CentralPanel::default().show(ctx, |ui| {
            for image in &mut self.images {
                image.update(ui, &self.view_state);
            }
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }

                    if ui.button("Import Image").clicked() {
                        self.load_image(ctx, &rfd::FileDialog::new().pick_file());
                    }
                });
            });
        });

        ctx.input(|input| {
            self.view_state.zoom *= input.zoom_delta();
            for file in &input.raw.dropped_files {
                self.load_image(ctx, &file.path);
            }

            if input.pointer.middle_down() || input.pointer.secondary_down() {
                self.view_state.offset += input.pointer.delta();
            }
        });
    }
}
