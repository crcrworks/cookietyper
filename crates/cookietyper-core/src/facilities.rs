use crate::game_state::GameState;

pub(crate) mod cursor;

pub(crate) trait Facility {
    fn on_purchase(&self) {}
    fn on_sell(&self) {}
    fn on_tick(&self, game_state: &mut GameState) {}

    fn can_purchase(&self, game_state: &GameState) -> bool {
        const EXP_BASE: f64 = 1.15;
        const STARTER_KITS: i32 = 0;
        EXP_BASE.powi(self.amount() as i32 - STARTER_KITS) * self.base_cost() as f64
            <= game_state.current_cookies() as f64
    }

    fn visual_state(&self) -> FacilityVisualState;
    fn amount(&self) -> u32;
    fn base_cost(&self) -> u128;
}

#[derive(Hash, PartialEq, Eq)]
pub(crate) enum FacilityKey {
    Cursor,
}

pub(crate) enum FacilityVisualState {
    Hidden,
    Covered,
    Displayed,
}
