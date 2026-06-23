use crate::data::{Data, Session};

#[derive(Default)]
pub struct WorkButton;
impl WorkButton {
    pub fn display(&mut self, ui: &mut egui::Ui, data: &mut Data) {
        if ui
            .button(egui::RichText::new("Work").color(data.work_color))
            .clicked()
        {
            data.reset_with_new_user_input = true;
            data.pause = true;
            data.session = Session::Work;
        }
    }
}
