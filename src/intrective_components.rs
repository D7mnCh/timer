pub mod pause_button;
pub mod reset_totals;
pub mod rest_button;
pub mod rest_secs_glider;
pub mod work_button;
pub mod work_secs_glider;

// crate begin with the this IntrComp directory ?, i think cause have the same name ?
use crate::PauseButton;
use crate::ResetTotals;
use crate::RestButton;
use crate::RestSecsGlider;
use crate::WorkButton;
use crate::WorkSecsGlider;

// crate begin from root here ? cause this is true ?

#[derive(Default)]
pub struct IntrComp {
    pub work_button: WorkButton,
    pub rest_button: RestButton,
    pub pause_button: PauseButton,
    pub work_secs_glider: WorkSecsGlider,
    pub rest_secs_glider: RestSecsGlider,
    pub reset_totals: ResetTotals,
}
