use std::collections::HashMap;

use crate::facilities::{Facility, FacilityKey, cursor::Cursor};

pub(crate) struct GameState {
    cpt: i128,
    current_cookies: u128,
    facilities: HashMap<FacilityKey, Box<dyn Facility>>,
}

impl GameState {
    pub(crate) fn current_cookies(&self) -> u128 {
        self.current_cookies
    }
}

impl Default for GameState {
    fn default() -> Self {
        let facilities: [(FacilityKey, Box<dyn Facility>); _] =
            [(FacilityKey::Cursor, Box::new(Cursor::default()))];

        Self {
            cpt: 1,
            current_cookies: 0,
            facilities: HashMap::from(facilities),
        }
    }
}
