use bnum::{
    cast::{As, CastFrom as _},
    types::{I512, U512},
};
use std::collections::HashMap;

macro_rules! facilities {
    ($($snake:ident :: $pascal:ident),* $(,)?) => {
        $(
            pub(crate) mod $snake;
        )*

        $(
            use crate::facilities::$snake::$pascal;
        )*

        #[derive(Hash, PartialEq, Eq)]
        pub(crate) enum FacilityKey {
            $(
                $pascal,
            )*
        }

        impl Default for Facilities {
            fn default() -> Self {
                let facilities = [
                    $(
                        $pascal::entry(),
                    )*
                ];
                Self {
                    inner: HashMap::from(facilities),
                    multiplier: 1.0,
                }
            }
        }
    };
}

facilities!(cursor::Cursor, grandma::Grandma);

pub(crate) trait Facility: FacilityHandlers + FacilityStatus + FacilityProperties {}

impl<T> Facility for T where T: FacilityHandlers + FacilityStatus + FacilityProperties {}

pub(crate) trait FacilityHandlers {
    fn on_purchase(&self) {}
    fn on_sell(&self) {}
    fn on_tick(&self, current_cookies: &mut U512) {}
}

pub(crate) trait FacilityStatus {
    fn visual_state(&self) -> FacilityVisualState;
}

pub(crate) trait FacilityProperties {
    fn key() -> FacilityKey
    where
        Self: Sized;

    fn entry() -> (FacilityKey, Box<dyn Facility>)
    where
        Self: FacilityHandlers + FacilityStatus + Default + 'static,
    {
        (Self::key(), Box::new(Self::default()))
    }

    fn can_purchase(&self, current_cookies: U512) -> bool {
        const EXP_BASE: f64 = 1.15;
        const STARTER_KITS: i32 = 0;
        EXP_BASE.powi(self.amount() as i32 - STARTER_KITS) * self.base_cost().as_::<f64>()
            <= current_cookies.as_::<f64>()
    }

    fn amount(&self) -> u32;
    fn multiplier(&self) -> f64;
    fn base_cost(&self) -> U512;
    fn base_cps(&self) -> f64;

    fn cps(&self) -> I512 {
        let cps = self.base_cps() * self.multiplier();
        I512::cast_from(cps)
    }
}

pub(crate) struct Facilities {
    inner: HashMap<FacilityKey, Box<dyn Facility>>,
    multiplier: f64,
}

impl Facilities {
    pub(crate) fn displayed(&self) -> Vec<&dyn Facility> {
        self.inner
            .values()
            .filter(|f| f.visual_state() == FacilityVisualState::Displayed)
            .map(|v| &**v)
            .collect()
    }

    pub(crate) fn total_cps(&self) -> I512 {
        self.displayed()
            .iter()
            .fold(I512::ZERO, |sum, facility| sum + facility.cps())
    }
}

#[derive(PartialEq, Eq)]
pub(crate) enum FacilityVisualState {
    Hidden,
    Covered,
    Displayed,
}
