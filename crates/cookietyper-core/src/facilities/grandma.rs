use crate::facilities::{Facility, FacilityKey, FacilityVisualState};

pub(crate) struct Grandma {
    amount: u32,
    multiplier: f64,
}

impl Default for Grandma {
    fn default() -> Self {
        Self {
            amount: 0,
            multiplier: 1.0,
        }
    }
}

impl Facility for Grandma {
    fn key() -> FacilityKey {
        FacilityKey::Grandma
    }

    fn visual_state(&self) -> FacilityVisualState {
        FacilityVisualState::Covered
    }

    fn amount(&self) -> u32 {
        self.amount
    }

    fn multiplier(&self) -> f64 {
        self.multiplier
    }

    fn base_cost(&self) -> bnum::types::U512 {
        100u32.into()
    }

    fn base_cps(&self) -> f64 {
        1.0
    }
}
