use crate::data::*;

#[derive(Default, Debug)]
pub struct SwitchCell {
    work_secs: u64,
    rest_secs: u64,
    mins: u64,
    hours: u64,
    temp_work_secs: u64,
    temp_rest_secs: u64,
}

impl SwitchCell {
    fn check_if_new_user_input(&mut self, data: &mut Data) {
        if data.reset_with_new_user_input {
            self.temp_work_secs = 0;
            self.temp_rest_secs = 0;
            data.reset_with_new_user_input = false;
        }
    }

    fn update_time(&mut self, data: &mut Data) {
        match data.session {
            Session::Work => {
                self.mins = self.work_secs / 60;
                self.hours = self.work_secs / (60 * 60);

                if !data.pause {
                    self.work_secs = self.temp_work_secs;

                    if self.work_secs != data.work_secs {
                        self.work_secs += data.instant.elapsed().as_secs();
                        // store temp to use it later, cuz when swithing to pause
                        // i'll change self.work_secs
                        self.temp_work_secs = self.work_secs;
                    } else {
                        data.pause = true;

                        //play sound
                        data.command = Command::PlaySound;
                        data.sound = Sound::MainRoundFinished;
                        data.command
                            .process_with(&mut data.child_process, &mut data.sound);

                        data.reset_with_new_user_input = true;
                    }
                } else {
                    self.check_if_new_user_input(data);
                    let remaining = data.work_secs - self.temp_work_secs;
                    self.work_secs = remaining;
                }
            }

            Session::Rest => {
                self.mins = self.rest_secs / 60;
                self.hours = self.rest_secs / (60 * 60);
                if !data.pause {
                    self.rest_secs = self.temp_rest_secs;

                    if self.rest_secs != data.rest_secs {
                        self.rest_secs += data.instant.elapsed().as_secs();
                        // store temp to use it later, cuz when swithing to pause
                        // i'll change self.reset_secs
                        self.temp_rest_secs = self.rest_secs;
                    } else {
                        data.pause = true;

                        //play sound
                        data.sound = Sound::Rest;
                        data.command = Command::PlaySound;
                        data.command
                            .process_with(&mut data.child_process, &mut data.sound);

                        data.reset_with_new_user_input = true;
                    }
                } else {
                    self.check_if_new_user_input(data);
                    let remaining = data.rest_secs - self.temp_rest_secs;
                    self.rest_secs = remaining;
                }
            }
        }
    }

    pub fn display(&mut self, ui: &mut egui::Ui, data: &mut Data) {
        self.update_time(data);

        let (color, secs) = match data.session {
            Session::Work => (data.work_color, self.work_secs),
            Session::Rest => (data.rest_color, self.rest_secs),
        };

        let degital_clock = format!(
            "{:02}:{:02}:{:02}",
            self.hours % 24,
            self.mins % 60,
            secs % 60,
        );

        let mut cell_size;
        if data.pause == false {
            cell_size = 80.0;
        } else {
            cell_size = 28.0;
        }

        ui.label(
            egui::RichText::new(degital_clock)
                .color(color)
                .strong()
                .size(cell_size),
        );
    }
}
