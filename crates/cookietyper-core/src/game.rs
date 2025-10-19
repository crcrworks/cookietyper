use chrono::{DateTime, Local};
use color_eyre::eyre;

use crate::facilities::Facilities;

/// The main game state structure
pub struct Game {
    cpt: f64,
    global_multiplier: f64,
    cpt_multiplier: f64,
    current_cookies: f64,
    facilities: Facilities,
    last_frame_time: DateTime<Local>,
}

impl Game {
    /// update game tick by tick
    pub fn update(&mut self) {
        let time_delta = (self.last_frame_time - Local::now()).abs();

        self.current_cookies += self.facilities.delta_cookies(time_delta);

        self.last_frame_time = Local::now();
    }

    /// earn cookies by calculating base cps and several multipliers
    pub fn earn_cookies(&mut self) {
        self.current_cookies += self.cpt * self.global_multiplier * self.cpt_multiplier;
    }

    pub fn current_cookies(&self) -> f64 {
        self.current_cookies
    }

    pub fn cps(&self) -> f64 {
        self.facilities.total_cps()
    }

    pub fn purchase_facility(&mut self) -> eyre::Result<()> {
        self.facilities.purchase(&mut self.current_cookies)
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            cpt: 1.0,
            global_multiplier: 1.0,
            cpt_multiplier: 1.0,
            current_cookies: 0.0,
            last_frame_time: Local::now(),
            facilities: Facilities::default(),
        }
    }
}
