use schemars::JsonSchema;
use serde::Deserialize;

use crate::boundary::OptionalBoundary;

#[derive(Deserialize, Debug, JsonSchema)]
pub struct ScoringData {
    pub vehicle_score: OptionalBoundary,
    pub loan_rate: f32,
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct PassengerBonuses {
    pub passenger_count: u8,
    pub bonus: f32,
}
