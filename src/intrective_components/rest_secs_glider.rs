use crate::Data;
use crate::Session;

#[derive(Default)]
pub struct RestSecsGlider {
    rest_mins: u64,
}
impl RestSecsGlider {
    fn secs_converter(&mut self, data: &mut Data) {
        data.rest_secs = self.rest_mins * 60;
    }

    fn get_user_saved_input(&mut self, data: &mut Data) {
        self.rest_mins = data.rest_secs / 60;
    }

    pub fn display(&mut self, ui: &mut egui::Ui, data: &mut Data) {
        self.get_user_saved_input(data);

        let response = ui.add(
            egui::DragValue::new(&mut self.rest_mins)
                .speed(0.1)
                .range(1.0..=120.0),
        );
        if response.dragged() {
            data.session = Session::Rest;
            data.reset_with_new_user_input = true;
        }
        if response.lost_focus() {
            data.reset_with_new_user_input = true;
        }

        self.secs_converter(data);
    }
}
