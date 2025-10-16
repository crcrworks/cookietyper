use crate::facilities::Facilities;

/// The main game state structure
pub struct Game {
    cpt: i128,
    current_cookies: u128,
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
            cpt: 1,
            current_cookies: 0,
            facilities: Facilities::default(),
        }
    }
}
