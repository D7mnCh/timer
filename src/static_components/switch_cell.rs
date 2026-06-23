use crate::data::*;

#[derive(Default, Debug)]
pub struct SwitchCell {
    work_secs: u64,
    rest_secs: u64,
    mins: u64,
    hours: u64,
}

impl SwitchCell {
    fn get_new_user_input(&mut self, data: &mut Data) {
        self.work_secs = data.work_secs;
        self.rest_secs = data.rest_secs;
    }

    fn update_time(&mut self, data: &mut Data) {
        match data.session {
            Session::Work => {
                self.mins = self.work_secs / 60;
                self.hours = self.work_secs / (60 * 60);
                if !data.pause {
                    if self.work_secs > 0 {
                        self.work_secs -= data.instant.elapsed().as_secs();
                    } else {
                        data.pause = true;
                        //play sound
                        data.command = Command::PlaySound;
                        data.sound = Sound::MainRoundFinished;
                        data.command
                            .process_with(&mut data.child_process, &mut data.sound);

                        data.reset_with_new_user_input = true;
                    }
                }
            }

            Session::Rest => {
                self.mins = self.rest_secs / 60;
                self.hours = self.rest_secs / (60 * 60);
                if !data.pause {
                    if self.rest_secs > 0 {
                        self.rest_secs -= data.instant.elapsed().as_secs();
                    } else {
                        data.pause = true;

                        //play sound
                        data.sound = Sound::Rest;
                        data.command = Command::PlaySound;
                        data.command
                            .process_with(&mut data.child_process, &mut data.sound);

                        data.reset_with_new_user_input = true;
                    }
                }
            }
        }
    }

    pub fn display(&mut self, ui: &mut egui::Ui, data: &mut Data) {
        if data.reset_with_new_user_input == true {
            self.get_new_user_input(data);
            data.reset_with_new_user_input = false;
        }

        self.update_time(data);

        let (color, which_session, secs) = match data.session {
            Session::Work => (data.work_color, "Work", self.work_secs),
            Session::Rest => (data.rest_color, "Rest", self.rest_secs),
        };
        let degital_clock = format!(
            "{} session: {:02}:{:02}:{:02}",
            which_session,
            self.hours % 24,
            self.mins % 60,
            secs % 60,
        );

        let mut cell_size;
        if data.pause == false {
            cell_size = 60.0;
        } else {
            cell_size = 16.0;
        }

        ui.label(
            egui::RichText::new(degital_clock)
                .color(color)
                .strong()
                .size(cell_size),
        );
    }
}
