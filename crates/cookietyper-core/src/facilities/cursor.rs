use crate::facilities::{FacilityHandlers, FacilityStatus, FacilityVisualState};
use bnum::types::U512;
use cookietyper_macro::Facility;

#[derive(Facility)]
#[facility(key = "Cursor", base_cost = 15, base_cps = 0.1)]
pub(crate) struct Cursor {
    amount: u32,
    multiplier: f64,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            amount: 0,
            multiplier: 1.0,
        }
    }
}

impl FacilityStatus for Cursor {
    fn visual_state(&self) -> FacilityVisualState {
        FacilityVisualState::Covered
    }
}

impl FacilityHandlers for Cursor {}
