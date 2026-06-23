#![allow(warnings)]
mod data;
use crate::data::{Command, Data, Session, Sound};

mod static_components;
use crate::static_components::rest_cell::*;
use crate::static_components::switch_cell::*;
use crate::static_components::work_cell::*;
use crate::static_components::*;

mod intrective_components;
use crate::intrective_components::pause_button::*;
use crate::intrective_components::reset_totals::*;
use crate::intrective_components::rest_button::*;
use crate::intrective_components::rest_secs_glider::*;
use crate::intrective_components::work_button::*;
use crate::intrective_components::work_secs_glider::*;
use crate::intrective_components::IntrComp;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        //[370, 115] tiny floating window
        viewport: egui::ViewportBuilder {
            inner_size: Some(egui::Vec2 {
                x: 1150.0,
                y: 170.0,
            }),
            title: Some("Timer".to_owned()),
            resizable: Some(false),
            ..Default::default()
        },
        ..Default::default()
    };
    eframe::run_native(
        "Timer",
        native_options,
        Box::new(|cc| Ok(Box::new(State::new(cc)))),
    )
}
struct State {
    static_comp: StaticComp,
    intr_comp: IntrComp,
    data: Data,
}
impl State {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            data: Data {
                instant: std::time::Instant::now(),
                reset_with_new_user_input: true,
                reset_totals: false,
                pause: true,
                sound: Sound::MainRoundFinished,
                session: Session::Work,
                command: Command::None,
                child_process: None,
                rest_secs: 3,
                work_secs: 3,
                work_color: egui::Color32::RED,
                rest_color: egui::Color32::GREEN,
            },
            static_comp: StaticComp {
                ..Default::default()
            },
            intr_comp: IntrComp {
                ..Default::default()
            },
        }
    }
}
impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                if !self.data.pause {
                    ui.add_space(40.0);
                    ui.horizontal(|ui| {
                        ui.add_space(460.0);
                        self.static_comp.switch_cell.display(ui, &mut self.data);
                    });

                    ui.add_space(25.0);

                    ui.horizontal(|ui| {
                        ui.add_space(525.0);
                        self.intr_comp.pause_button.display(ui, &mut self.data);
                    });
                } else {
                    ui.horizontal(|ui| {
                        ui.add_space(540.0);
                        self.static_comp.switch_cell.display(ui, &mut self.data);
                    });

                    ui.add_space(7.0);

                    ui.horizontal(|ui| {
                        ui.add_space(435.0);
                        ui.group(|ui| {
                            ui.add_space(75.0);
                            ui.vertical(|ui| {
                                self.static_comp.work_cell.display(ui, &mut self.data);
                                ui.add_space(5.0);
                                self.static_comp.rest_cell.display(ui, &mut self.data);
                            });
                            ui.add_space(75.0);
                        });
                    });
                    if self.data.reset_totals == true {
                        self.data.reset_totals = false
                    }

                    // NOTE intrective_components starts to show here !

                    ui.horizontal(|ui| {
                        ui.add_space(530.0);
                        self.intr_comp.pause_button.display(ui, &mut self.data);
                    });
                    ui.horizontal(|ui| {
                        ui.add_space(435.0);
                        ui.vertical(|ui| {
                            self.intr_comp.rest_button.display(ui, &mut self.data);
                            self.intr_comp.rest_secs_glider.display(ui, &mut self.data);
                        });
                        ui.add_space(70.0);
                        self.intr_comp.reset_totals.display(ui, &mut self.data);
                        ui.add_space(65.0);
                        ui.vertical(|ui| {
                            self.intr_comp.work_button.display(ui, &mut self.data);
                            self.intr_comp.work_secs_glider.display(ui, &mut self.data);
                        });
                    });
                }
            });
        });
        //println!("{}", ctx.viewport_rect().max);
        // if device suspended, self.data.instant.elapsed().as_secs() will get
        // more then 1
        if self.data.instant.elapsed().as_secs() > 1 {
            self.data.pause = true;
            self.data.instant = std::time::Instant::now();
        }
        if self.data.instant.elapsed().as_secs() == 1 {
            self.data.instant = std::time::Instant::now();
        }
    }
}
/*

 TODO
 - sometimes when i finshed the 45 mins it will not add 15 mins, i think this happen when i pause and repause
 - don't show totals, make remaining underneath the big timer so only three things will get displayed
 - after timer finished, reset the timer, if no it will be 0, and the user
 after that can spam play to spawn sounds multiple times
 - i want to press a key that allows to fast finishing the timer
 - for my app, just pkg with a directory that have sounds directory on it just it
 - fix path issue when running with dmenue
 - when i use dragging cell, if i pressed Ok then get the new updated secs ! (use response
 struct i guess)
 - display whech round i am (main or bonus)
 - keyboard support
 - web support
 - IO improvment
 - add click sounds, so i need to seperate clicking sounds and the main sounds when finished timer
 - switch to 15 minutes after 45 minutes is done (don't make 15 minutes on
  rest session)
 - when the sound finished, make the buttom to disapear
 NOTE (bugs)
 - one functino have different functianlity
 - you made ui and logic in the same function -.-
 - no ui mod
 - sometimes the timer will freeze for one second or even
 - the program you are building is for you after all, the features you will add
 is based on your needs
 - if you exit the app with the midia player being played it will hide the cursor
 when running on the terminal
    - `reset` to make cursor appear again
*/
