use lingual_number::LingNum;

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

#[derive(Default)]
struct FacilityBuilder {
    base_cps: Option<f64>,
    base_cost: Option<f64>,
}

impl FacilityBuilder {
    fn base_cps(mut self, base_cps: f64) -> Self {
        self.base_cps = Some(base_cps);
        self
    }

    fn base_cost(mut self, base_cost: f64) -> Self {
        self.base_cost = Some(base_cost);
        self
    }

    fn build(self) -> Facility {
        Facility {
            amount: 0,
            visual_state: FacilityVisualState::Hidden,
            multiplier: 1.0,
            base_cps: self.base_cps.unwrap_or(1.0),
            base_cost: self.base_cost.unwrap_or(100.0),
        }
    }
}

pub(crate) struct Facilities {
    inner: [Facility; 20],
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
        let cursor = FacilityBuilder::default().base_cps(0.1).base_cost(15.0);
        let grandma = FacilityBuilder::default().base_cps(1.0).base_cost(100.0);
        let farm = FacilityBuilder::default().base_cps(8.0).base_cost(1_100.0);
        let mine = FacilityBuilder::default()
            .base_cps(47.0)
            .base_cost(12_000.0);
        let factory = FacilityBuilder::default()
            .base_cps(260.0)
            .base_cost(130_000.0);
        let bank = FacilityBuilder::default()
            .base_cps(1_400.0)
            .base_cost(LingNum::new(1.4).million());
        let temple = FacilityBuilder::default()
            .base_cps(7_800.0)
            .base_cost(LingNum::new(20.0).million());
        let wizard_tower = FacilityBuilder::default()
            .base_cps(44_000.0)
            .base_cost(LingNum::new(330.0).million());
        let shipment = FacilityBuilder::default()
            .base_cps(260_000.0)
            .base_cost(LingNum::new(5.1).billion());
        let alchemy_lab = FacilityBuilder::default()
            .base_cps(LingNum::new(1.6).million())
            .base_cost(LingNum::new(75.0).billion());
        let portal = FacilityBuilder::default()
            .base_cps(LingNum::new(10.0).million())
            .base_cost(LingNum::new(1.0).trillion());
        let time_machine = FacilityBuilder::default()
            .base_cps(LingNum::new(65.0).million())
            .base_cost(LingNum::new(14.0).trillion());
        let antimatter_condenser = FacilityBuilder::default()
            .base_cps(LingNum::new(430.0).million())
            .base_cost(LingNum::new(170.0).trillion());
        let prism = FacilityBuilder::default()
            .base_cps(LingNum::new(2.9).billion())
            .base_cost(LingNum::new(2.1).quadrillion());
        let chancemaker = FacilityBuilder::default()
            .base_cps(LingNum::new(21.0).billion())
            .base_cost(LingNum::new(26.0).quadrillion());
        let fractal_engine = FacilityBuilder::default()
            .base_cps(LingNum::new(150.0).billion())
            .base_cost(LingNum::new(310.0).quadrillion());
        let javascript_console = FacilityBuilder::default()
            .base_cps(LingNum::new(1.1).trillion())
            .base_cost(LingNum::new(71.0).quintillion());
        let idleverse = FacilityBuilder::default()
            .base_cps(LingNum::new(8.3).trillion())
            .base_cost(LingNum::new(12.0).sextillion());
        let cortex_baker = FacilityBuilder::default()
            .base_cps(LingNum::new(64.0).trillion())
            .base_cost(LingNum::new(1.9).septillion());
        let you = FacilityBuilder::default()
            .base_cps(LingNum::new(510.0).trillion())
            .base_cost(LingNum::new(540.0).septillion());

        Self {
            inner: [
                cursor,
                grandma,
                farm,
                mine,
                factory,
                bank,
                temple,
                wizard_tower,
                shipment,
                alchemy_lab,
                portal,
                time_machine,
                antimatter_condenser,
                prism,
                chancemaker,
                fractal_engine,
                javascript_console,
                idleverse,
                cortex_baker,
                you,
            ]
            .map(|f| f.build()),
            multiplier: 1.0,
        }
    }
}
