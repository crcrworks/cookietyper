use bnum::{
    cast::{As, CastFrom},
    types::{I512, U512},
};

use crate::facilities::Facilities;

/// The main game state structure
pub struct Game {
    cpt: I512,
    global_multiplier: f64,
    cpt_multiplier: f64,
    current_cookies: U512,
    facilities: Facilities,
}

impl Game {
    fn update(&mut self) {
        let facilities = self.facilities.displayed();

        for facility in facilities {
            facility.on_tick(&mut self.current_cookies);
        }
    }

    fn earn_cookies(&mut self) {
        let right = self.cpt.as_::<f64>() * self.cpt_multiplier;
        self.current_cookies += U512::cast_from(right);
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            cpt: I512::from(1),
            global_multiplier: 1.0,
            cpt_multiplier: 1.0,
            current_cookies: U512::from(0u32),
            facilities: Facilities::default(),
        }
    }
}
