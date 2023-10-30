use std::collections::HashMap;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::boundary::OptionalBoundary;

#[derive(Deserialize, JsonSchema)]
pub struct ScoringData {
    pub vehicle_score: OptionalBoundary,
    pub loan_rate: f32,
}

#[derive(Deserialize, JsonSchema)]
pub struct PassengerBonuses {
    #[serde(flatten)]
    pub bonus: HashMap<u8, f32>,
}
