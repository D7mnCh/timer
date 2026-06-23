#[derive(Debug)]
pub enum Session {
    Work,
    Rest,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    PlaySound,
    None,
}
#[derive(Debug, PartialEq)]
pub enum Sound {
    MainRoundFinished,
    BonusRoundFinished,
    Rest,
}
#[derive(Debug)]
pub struct Data {
    // NOTE maybe i can remove this field
    pub reset_with_new_user_input: bool,
    pub pause: bool,
    pub reset_totals: bool,
    pub instant: std::time::Instant,
    pub sound: Sound,
    pub session: Session,
    pub command: Command,
    pub child_process: Option<std::process::Child>,
    pub rest_secs: u64,
    pub work_secs: u64,
    pub work_color: egui::Color32,
    pub rest_color: egui::Color32,
}

impl Command {
    fn play_sound(child_process: &mut Option<std::process::Child>, sound: &mut Sound) {
        let path = std::env::current_dir()
            .expect("couldn't find sounds directory on the current directory")
            .join("sounds");
        //std::process::Command::new("dunstify")
        //    .arg(&path)
        //    .spawn()
        //    .unwrap();
        //let path = "/usr/local/bin/Timer/sounds";
        // which sound to play
        let sound: &str = match sound {
            Sound::MainRoundFinished => "main_round.wav",
            Sound::BonusRoundFinished => "bonus_round.wav",
            Sound::Rest => "end_rest.wav",
        };

        #[cfg(target_family = "unix")]
        {
            let command = std::process::Command::new("mpv")
                .current_dir(path)
                // NOTE if program crash, still show me the terminal cursor
                .args([sound, "--no-audio-display", "--no-terminal"])
                .spawn()
                .expect("mpv failed to lunch");
            *child_process = Some(command);
        }

        #[cfg(target_family = "windows")]
        {
            let command_arg = format!("(New-Object Media.SoundPlayer '.\\{}').PlaySync();", sound);
            let command = std::process::Command::new("powershell")
                .current_dir(path)
                .args([
                    "-c",
                    // NOTE to change
                    &command_arg,
                ])
                .spawn()
                .expect("powershell failed to start or the problem could be in args");

            *child_process = Some(command);
        }
    }

    //NOTE the args will grow while the commands enum grow
    pub fn process_with(&self, child_process: &mut Option<std::process::Child>, sound: &mut Sound) {
        match self {
            Command::PlaySound => {
                println!("sound on");
                Self::play_sound(child_process, sound);
            }
            Command::None => println!("No command at the moment"),
        }
    }
}
