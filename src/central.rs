#![no_main]
#![no_std]
extern crate self as my_crate;
pub use custom_renderer::SimpleStatsRenderer as DongleRenderer;

mod pointingproccontroller;

use rmk::macros::rmk_central;

#[rmk_central]
mod keyboard_central {
    #[register_processor(event)]
    fn pointing_processor_controller() -> crate::pointingproccontroller::PointingProcessorController {
        crate::pointingproccontroller::PointingProcessorController::new()
    }
}

