use crate::facilities::{Facility, FacilityVisualState};

pub(crate) struct Cursor {
    amount: u32,
    multiplier: f64,
}

impl Cursor {
    const BASE_CPS: f64 = 0.1;
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            amount: 0,
            multiplier: 1.0,
        }
    }
}

impl Facility for Cursor {
    fn visual_state(&self) -> FacilityVisualState {
        FacilityVisualState::Displayed
    }

    fn amount(&self) -> u32 {
        self.amount
    }

    fn base_cost(&self) -> u128 {
        15
    }
}
