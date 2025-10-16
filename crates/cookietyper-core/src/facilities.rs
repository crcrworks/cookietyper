use std::collections::HashMap;

use crate::facilities::cursor::Cursor;
pub(crate) mod cursor;

pub(crate) trait Facility {
    fn on_purchase(&self) {}
    fn on_sell(&self) {}
    fn on_tick(&self, current_cookies: &mut u128) {}

    fn can_purchase(&self, current_cookies: u128) -> bool {
        const EXP_BASE: f64 = 1.15;
        const STARTER_KITS: i32 = 0;
        EXP_BASE.powi(self.amount() as i32 - STARTER_KITS) * self.base_cost() as f64
            <= current_cookies as f64
    }

    fn visual_state(&self) -> FacilityVisualState;
    fn amount(&self) -> u32;
    fn base_cost(&self) -> u128;
}

pub(crate) struct Facilities(HashMap<FacilityKey, Box<dyn Facility>>);

impl Facilities {
    pub(crate) fn displayed(&self) -> Vec<&dyn Facility> {
        self.0
            .values()
            .filter(|f| f.visual_state() == FacilityVisualState::Displayed)
            .map(|v| &**v)
            .collect()
    }
}

impl Default for Facilities {
    fn default() -> Self {
        let facilities: [(FacilityKey, Box<dyn Facility>); _] =
            [(FacilityKey::Cursor, Box::new(Cursor::default()))];

        Self(HashMap::from(facilities))
    }
}

#[derive(Hash, PartialEq, Eq)]
pub(crate) enum FacilityKey {
    Cursor,
}

#[derive(PartialEq, Eq)]
pub(crate) enum FacilityVisualState {
    Hidden,
    Covered,
    Displayed,
}
