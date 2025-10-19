use crate::facilities::Facilities;

/// The main game state structure
pub struct Game {
    cpt: f64,
    global_multiplier: f64,
    cpt_multiplier: f64,
    current_cookies: f64,
    facilities: Facilities,
}

impl Game {
    /// update game tick by tick
    pub fn update(&mut self) {
        self.facilities.update_all();
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
}

impl Default for Game {
    fn default() -> Self {
        Self {
            cpt: 1.0,
            global_multiplier: 1.0,
            cpt_multiplier: 1.0,
            current_cookies: 0.0,
            facilities: Facilities::default(),
        }
    }
}
