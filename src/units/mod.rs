pub mod app;
// pub mod result_block;
// pub mod results;
// pub mod library;
// pub mod objective_block;
pub mod side_menu;
pub mod top_menu;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}
