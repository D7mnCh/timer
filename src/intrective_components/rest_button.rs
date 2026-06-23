use crate::Data;
use crate::Session;
use egui::Ui;

#[derive(Default)]
pub struct RestButton;
impl RestButton {
    pub fn display(&mut self, ui: &mut Ui, data: &mut Data) {
        if ui
            .button(egui::RichText::new(" Rest").color(data.rest_color))
            .clicked()
        {
            //data.reset = true;
            data.reset_with_new_user_input = true;
            data.pause = true;
            data.session = Session::Rest;
        }
    }
}
