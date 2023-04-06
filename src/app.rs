#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct PlinitApp {
    #[serde(skip)]
    textures: Vec<egui::TextureHandle>,
}

impl PlinitApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        let image = image::open("mpv-shot0001.jpg").unwrap();

        Self::default()
    }

    fn load_image(&mut self, ctx: &egui::Context, path: &str) -> image::ImageResult<()> {
        let image = image::open(path)?;
        let texture = ctx.load_texture(
            path,
            egui::ColorImage::from_rgb(
                [image.width() as usize, image.height() as usize],
                image.as_bytes(),
            ),
            Default::default(),
        );

        self.textures.push(texture);

        Ok(())
    }
}

impl eframe::App for PlinitApp {
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }

                    if ui.button("Import Image").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            let path = path.to_str().unwrap_or("");
                            let result = self.load_image(ctx, path);
                            if let Err(err) = result {
                                println!("Failed to load {} - {}", path, err);
                            }
                        }
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            for texture in &self.textures {
                ui.image(texture, texture.size_vec2() / 2.0);
            }

            egui::warn_if_debug_build(ui);
        });
    }
}
