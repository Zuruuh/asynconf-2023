use schemars::JsonSchema;
use serde::Deserialize;

mod bank;
mod boundary;
mod vehicle;

#[derive(Deserialize, JsonSchema)]
pub struct Config {
    pub vehicle_data: Vec<vehicle::Vehicle>,
    pub energy_data: Vec<vehicle::Energy>,
    pub mileage_data: Vec<vehicle::Mileage>,
    pub generation_data: Vec<vehicle::Generation>,
    pub banking_data: Vec<bank::ScoringData>,
    pub passenger_bonuses: bank::PassengerBonuses,
}
