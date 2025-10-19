use crate::facilities::{FacilityHandlers, FacilityStatus};
use bnum::types::U512;
use cookietyper_macro::Facility;

#[derive(Facility)]
#[facility(base_cost = 100, base_cps = 1.0, key = "Grandma")]
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

impl FacilityStatus for Grandma {
    fn visual_state(&self) -> super::FacilityVisualState {
        todo!()
    }
}

impl FacilityHandlers for Grandma {}
