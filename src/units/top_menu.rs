// ----------------------------------------------------------------------------

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MenuBar {
    // logo: egui::Image,
    // texture: Option<egui::TextureHandle>,
}

impl MenuBar {}

impl Default for MenuBar {
    fn default() -> Self {
        // // Show the image:
        // ui.add(egui::Image::new(texture, texture.size_vec2()));

        Self {
            // logo: egui::Image::
        }
    }
}

impl super::View for MenuBar {
    fn ui(&mut self, ui: &mut egui::Ui) {
        // let texture: egui::TextureHandle =
        //     ui.ctx().load_texture(
        //         "my-image",
        //         egui::ColorImage::example(),
        //         Default::default()
        //     );
        //
        // ui.add_sized([250.0, 250.0],
        // egui::Image::new(&texture, [250.0, 250.0]));
        //
        //
        // ui.end_row();

        ui.separator();
    }
}
