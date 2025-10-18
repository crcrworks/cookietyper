use bnum::types::{I512, U512};

use crate::facilities::Facilities;

/// The main game state structure
pub struct Game {
    cpt: I512,
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
}

impl Default for Game {
    fn default() -> Self {
        Self {
            cpt: I512::from(1),
            current_cookies: U512::from(0u32),
            facilities: Facilities::default(),
        }
    }
}
