use crate::data::Data;

#[derive(Default)]
pub struct PauseButton;
impl PauseButton {
    pub fn display(&mut self, ui: &mut egui::Ui, data: &mut Data) {
        let text = if data.pause == false { "Pause" } else { "Play" };
        if ui.add_sized([120., 20.], egui::Button::new(text)).clicked() {
            data.pause = !data.pause;
        }
    }
}
