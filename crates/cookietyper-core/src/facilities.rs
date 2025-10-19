#[derive(PartialEq, Eq)]
pub(crate) enum FacilityVisualState {
    Hidden,
    Covered,
    Displayed,
}

pub(crate) struct Facility {
    base_cps: f64,
    base_cost: f64,
    amount: u32,
    visual_state: FacilityVisualState,
    multiplier: f64,
}

impl Facility {
    fn cps(&self) -> f64 {
        self.base_cps * self.multiplier * self.amount as f64
    }

    fn can_purchase(&self, current_cookies: f64) -> bool {
        const EXP_BASE: f64 = 1.15;
        const STARTER_KITS: u32 = 0;
        EXP_BASE.powi((self.amount - STARTER_KITS) as i32) * self.base_cost <= current_cookies
    }

    fn purchase(&mut self, price: f64, current_cookies: &mut f64) {
        *current_cookies -= price;
        self.amount += 1;
    }
}

pub(crate) struct Facilities {
    inner: [Facility; 2],
    multiplier: f64,
}

impl Facilities {
    pub(crate) fn update_all(&mut self) {}

    pub(crate) fn total_cps(&self) -> f64 {
        self.inner
            .iter()
            .filter(|facility| facility.visual_state == FacilityVisualState::Displayed)
            .fold(0.0, |sum, facility| sum + facility.cps())
            * self.multiplier
    }
}

impl Default for Facilities {
    fn default() -> Self {
        let cursor = Facilities::new().base_cps(10).base_cost(20);
        let grandma = Facilities::new();
        let farm = Facilities::new();
        let factory = Facilities::new();

        Self {
            inner: [cursor, grandma, farm, factory],
            multiplier: 1.0,
        }
    }
}
