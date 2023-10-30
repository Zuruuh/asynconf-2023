use crate::{vehicle::Vehicle, Config};
use std::rc::Rc;

pub struct Calculator {
    config: Rc<Config>,
    score_calculator: ScoreCalculators,
}

impl Calculator {
    pub fn new(config: Config) -> Self {
        let config = Rc::new(config);

        Self {
            config: config.clone(),
            score_calculator: ScoreCalculators::default(config),
        }
    }

    pub fn calculate_rate(&self, vehicle: Vehicle, passenger_count: u8) -> f32 {
        let score = self.score_calculator.calculate_for_vehicle(&vehicle);

        let loan_rate = self
            .config
            .banking_data
            .iter()
            .find(|rate| {
                rate.vehicle_score
                    .upper_boundary
                    .map(|val| val as f32)
                    .unwrap_or(f32::MAX)
                    >= score
                    && score
                        >= rate
                            .vehicle_score
                            .lower_boundary
                            .map(|val| val as f32)
                            .unwrap_or(0.0)
            })
            .map(|loan_rate| loan_rate.loan_rate)
            .unwrap();

        let bonus = self.calculate_bonus(passenger_count);

        loan_rate + bonus
    }

    fn calculate_bonus(&self, passenger_count: u8) -> f32 {
        match passenger_count {
            1 => 0.11,
            2 => -0.17,
            3 => -0.29,
            4 => -0.53,
            _ => unimplemented!(),
        }
    }
}

struct ScoreCalculators(Vec<Box<dyn ScoreCalculator>>);
impl ScoreCalculators {
    pub fn calculate_for_vehicle(&self, vehicle: &Vehicle) -> f32 {
        self.0
            .iter()
            .map(|calculator| calculator.calculate_score(vehicle))
            .sum::<f32>()
    }

    fn default(config: Rc<Config>) -> Self {
        Self(vec![
            Box::new(CarModelScoreCalculator(config.clone())),
            Box::new(EnergyScoreCalculator(config.clone())),
            Box::new(MileageScoreCalculator(config.clone())),
            Box::new(GenerationScoreCalculator(config.clone())),
        ])
    }
}

trait ScoreCalculator {
    fn calculate_score(&self, vehicle: &Vehicle) -> f32;
}

struct CarModelScoreCalculator(pub Rc<Config>);
impl ScoreCalculator for CarModelScoreCalculator {
    fn calculate_score(&self, vehicle: &Vehicle) -> f32 {
        self.0
            .vehicle_data
            .iter()
            .find(|data| data.name == vehicle.name)
            .map(|note| note.ecological_note.0)
            .unwrap()
    }
}

struct EnergyScoreCalculator(pub Rc<Config>);
impl ScoreCalculator for EnergyScoreCalculator {
    fn calculate_score(&self, vehicle: &Vehicle) -> f32 {
        self.0
            .energy_data
            .iter()
            .find(|data| data.name == vehicle.energy.0)
            .map(|note| note.ecological_note.0)
            .unwrap()
    }
}

struct MileageScoreCalculator(pub Rc<Config>);
impl ScoreCalculator for MileageScoreCalculator {
    fn calculate_score(&self, vehicle: &Vehicle) -> f32 {
        self.0
            .mileage_data
            .iter()
            .find(|data| {
                data.boundary.upper_boundary.unwrap_or(usize::MAX) > vehicle.mileage.0
                    && vehicle.mileage.0 >= data.boundary.lower_boundary.unwrap_or(0)
            })
            .map(|note| note.ecological_note.0)
            .unwrap()
    }
}

struct GenerationScoreCalculator(pub Rc<Config>);
impl ScoreCalculator for GenerationScoreCalculator {
    fn calculate_score(&self, vehicle: &Vehicle) -> f32 {
        self.0
            .generation_data
            .iter()
            .find(|data| {
                data.boundary.upper_boundary.unwrap_or(usize::MAX) > vehicle.generation.0
                    && vehicle.generation.0 >= data.boundary.lower_boundary.unwrap_or(0)
            })
            .map(|note| note.ecological_note.0)
            .unwrap()
    }
}
