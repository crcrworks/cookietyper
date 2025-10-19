use chrono::TimeDelta;
use color_eyre::eyre::{self, bail};
use lingual_number::LingNum as _;

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
    #[inline]
    fn cps(&self) -> f64 {
        self.base_cps * self.multiplier * self.amount as f64
    }

    #[inline]
    fn price(&self) -> f64 {
        const EXP_BASE: f64 = 1.15;
        const STARTER_KITS: u32 = 0;
        EXP_BASE.powi((self.amount - STARTER_KITS) as i32) * self.base_cost
    }

    #[inline]
    fn can_purchase(&self, current_cookies: f64) -> bool {
        self.price() <= current_cookies
    }

    #[inline]
    fn purchase(&mut self, current_cookies: &mut f64) -> eyre::Result<()> {
        if !self.can_purchase(*current_cookies) {
            bail!("Not enough cookies")
        }

        *current_cookies -= self.price();
        self.amount += 1;
        self.visual_state = FacilityVisualState::Displayed;
        Ok(())
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FacilityKey {
    Cursor,
    Grandma,
    Farm,
    Mine,
    Factory,
    Bank,
    Temple,
    WizardTower,
    Shipment,
    AlchemyLab,
    Portal,
    TimeMachine,
    AntimatterCondenser,
    Prism,
    Chancemaker,
    FractalEngine,
    JavascriptConsole,
    Idleverse,
    CortexBaker,
    You,
}

impl FacilityKey {
    #[inline]
    fn to_index(self) -> usize {
        match self {
            FacilityKey::Cursor => 0,
            FacilityKey::Grandma => 1,
            FacilityKey::Farm => 2,
            FacilityKey::Mine => 3,
            FacilityKey::Factory => 4,
            FacilityKey::Bank => 5,
            FacilityKey::Temple => 6,
            FacilityKey::WizardTower => 7,
            FacilityKey::Shipment => 8,
            FacilityKey::AlchemyLab => 9,
            FacilityKey::Portal => 10,
            FacilityKey::TimeMachine => 11,
            FacilityKey::AntimatterCondenser => 12,
            FacilityKey::Prism => 13,
            FacilityKey::Chancemaker => 14,
            FacilityKey::FractalEngine => 15,
            FacilityKey::JavascriptConsole => 16,
            FacilityKey::Idleverse => 17,
            FacilityKey::CortexBaker => 18,
            FacilityKey::You => 19,
        }
    }
}

pub(crate) struct Facilities {
    inner: [Facility; 20],
    multiplier: f64,
}

impl Facilities {
    pub(crate) fn delta_cookies(&mut self, time_delta: TimeDelta) -> f64 {
        let duration = time_delta.num_milliseconds() as f64 * 0.001;
        self.total_cps() * duration
    }

    pub(crate) fn total_cps(&self) -> f64 {
        self.inner
            .iter()
            .filter(|facility| facility.visual_state == FacilityVisualState::Displayed)
            .fold(0.0, |sum, facility| sum + facility.cps())
            * self.multiplier
    }

    pub fn purchase(
        &mut self,
        current_cookies: &mut f64,
        facility_key: FacilityKey,
    ) -> eyre::Result<()> {
        self.inner[facility_key.to_index()].purchase(current_cookies)
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
            .base_cost(1.4 * f64::MILLION);

        let temple = FacilityBuilder::default()
            .base_cps(7_800.0)
            .base_cost(20.0 * f64::MILLION);
        let wizard_tower = FacilityBuilder::default()
            .base_cps(44_000.0)
            .base_cost(330.0 * f64::MILLION);
        let shipment = FacilityBuilder::default()
            .base_cps(260_000.0)
            .base_cost(5.1 * f64::BILLION);
        let alchemy_lab = FacilityBuilder::default()
            .base_cps(1.6 * f64::MILLION)
            .base_cost(75.0 * f64::BILLION);
        let portal = FacilityBuilder::default()
            .base_cps(10.0 * f64::MILLION)
            .base_cost(1.0 * f64::TRILLION);
        let time_machine = FacilityBuilder::default()
            .base_cps(65.0 * f64::MILLION)
            .base_cost(14.0 * f64::TRILLION);
        let antimatter_condenser = FacilityBuilder::default()
            .base_cps(430.0 * f64::MILLION)
            .base_cost(170.0 * f64::TRILLION);
        let prism = FacilityBuilder::default()
            .base_cps(2.9 * f64::BILLION)
            .base_cost(2.1 * f64::QUADRILLION);
        let chancemaker = FacilityBuilder::default()
            .base_cps(21.0 * f64::BILLION)
            .base_cost(26.0 * f64::QUADRILLION);
        let fractal_engine = FacilityBuilder::default()
            .base_cps(150.0 * f64::BILLION)
            .base_cost(310.0 * f64::QUADRILLION);
        let javascript_console = FacilityBuilder::default()
            .base_cps(1.1 * f64::TRILLION)
            .base_cost(71.0 * f64::QUINTILLION);
        let idleverse = FacilityBuilder::default()
            .base_cps(8.3 * f64::TRILLION)
            .base_cost(12.0 * f64::SEXTILLION);
        let cortex_baker = FacilityBuilder::default()
            .base_cps(64.0 * f64::TRILLION)
            .base_cost(1.9 * f64::SEPTILLION);
        let you = FacilityBuilder::default()
            .base_cps(510.0 * f64::TRILLION)
            .base_cost(540.0 * f64::SEPTILLION);

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
