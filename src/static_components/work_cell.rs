use crate::Data;
use crate::Session;
use egui::*;

#[derive(Default)]
pub struct WorkCell {
    secs: u64,
    mins: u64,
    hours: u64,
}
impl WorkCell {
    fn update_time(&mut self) {
        self.mins = self.secs / 60;
        self.hours = self.secs / (60 * 60);
    }
    pub fn display(&mut self, ui: &mut Ui, data: &mut Data) {
        self.update_time();
        if data.reset_totals == true {
            self.secs = 0;
        }
        if data.pause == false {
            if let Session::Work = data.session {
                self.secs += data.instant.elapsed().as_secs();
            }
        }
        let degital_clock = format!(
            "Total work: {:02}:{:02}:{:02}",
            self.hours % 24,
            self.mins % 60,
            self.secs % 60
        );
        let degital_clock = degital_clock.as_str();

        ui.label(
            egui::RichText::new(degital_clock)
                .color(egui::Color32::DARK_RED)
                .size(15.0),
        );
    }
}
