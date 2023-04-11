pub struct PlinitImage {
    texture: egui::TextureHandle,
    pub position: egui::Pos2,
    pub rotation: f32,
}

impl PlinitImage {
    pub fn load(ctx: &egui::Context, path: &str) -> image::ImageResult<Self> {
        let image = image::open(path)?;
        let texture = ctx.load_texture(
            path,
            egui::ColorImage::from_rgb(
                [image.width() as usize, image.height() as usize],
                image.as_bytes(),
            ),
            Default::default(),
        );

        Ok(Self {
            texture,
            position: ctx.screen_rect().center(),
            rotation: 0.,
        })
    }

    pub fn update(&mut self, ui: &mut egui::Ui, view: &crate::ViewState) {
        let size = self.texture.size_vec2() * view.zoom;
        let position = self.position + view.offset;
        if ui
            .put(
                egui::Rect::from_center_size(position, size),
                egui::Image::new(self.texture.id(), size)
                    .rotate(f32::to_radians(self.rotation), egui::Vec2::splat(0.))
                    .sense(egui::Sense::click_and_drag()),
            )
            .dragged_by(egui::PointerButton::Primary)
        {
            if let Some(pos) = ui.ctx().pointer_latest_pos() {
                self.position = pos - view.offset;
            }
        }
    }
}
