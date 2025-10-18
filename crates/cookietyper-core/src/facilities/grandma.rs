use bnum::{cast::CastFrom as _, types::I512};

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

impl Grandma {
    const BASE_CPS: f64 = 1.0;
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

    fn base_cost(&self) -> bnum::types::U512 {
        100u32.into()
    }

    fn cps(&self) -> I512 {
        let cps = Grandma::BASE_CPS * self.multiplier;
        I512::cast_from(cps)
    }
}
