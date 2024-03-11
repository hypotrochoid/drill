use super::app::ViewMode;

// ----------------------------------------------------------------------------

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SideMenuBar {
    pub selected_mode: ViewMode,
}

impl Default for SideMenuBar {
    fn default() -> Self {
        Self {
            selected_mode: ViewMode::Flashcard,
        }
    }
}

impl super::View for SideMenuBar {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Mode");

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
            if ui.button("Flashcard").clicked() {
                self.selected_mode = ViewMode::Flashcard;
            }

            if ui.button("Dictionary").clicked() {
                self.selected_mode = ViewMode::Dictionary;
            }
        });
    }
}
