use schemars::JsonSchema;
use serde::Deserialize;

mod bank;
mod boundary;
pub mod calculator;
pub mod vehicle;

#[derive(Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Config {
    pub vehicle_data: Vec<vehicle::config::VehicleConfig>,
    pub energy_data: Vec<vehicle::config::EnergyConfig>,
    pub mileage_data: Vec<vehicle::config::MileageConfig>,
    pub generation_data: Vec<vehicle::config::GenerationConfig>,
    pub banking_data: Vec<bank::ScoringData>,
    pub passenger_bonuses: Vec<bank::PassengerBonuses>,
}

#[cfg(test)]
mod test {
    use crate::{
        calculator::Calculator,
        vehicle::{Energy, Generation, Mileage, Vehicle},
        Config,
    };

    #[test]
    pub fn example_one() {
        let default_data =
            serde_json::from_str::<Config>(std::include_str!("../../../data.json")).unwrap();
        let calculator = Calculator::new(default_data);

        let vehicles: Vec<(Vehicle, u8, f32)> = vec![
            (
                Vehicle::new(
                    "Citadine".into(),
                    Energy("Électrique".into()),
                    Mileage(25000),
                    Generation(2009),
                ),
                2,
                2.35,
            ),
            (
                Vehicle::new(
                    "Citadine".into(),
                    Energy("Hybride".into()),
                    Mileage(6000),
                    Generation(2015),
                ),
                4,
                1.57,
            ),
            (
                Vehicle::new(
                    "Cabriolet".into(),
                    Energy("Diesel".into()),
                    Mileage(14000),
                    Generation(1998),
                ),
                2,
                2.35,
            ),
        ];

        for (vehicle, passenger, expected_value) in vehicles {
            let bonus = calculator.calculate_rate(vehicle, passenger);

            assert_eq!((expected_value * 100.0).round(), (bonus * 100.0).round());
        }
    }
}
