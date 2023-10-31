use schemars::JsonSchema;
use serde::Deserialize;

pub mod config;

#[derive(PartialEq, Debug)]
pub struct Vehicle {
    pub name: String,
    pub energy: Energy,
    pub mileage: Mileage,
    pub generation: Generation,
}

impl Vehicle {
    pub fn new(name: String, energy: Energy, mileage: Mileage, generation: Generation) -> Self {
        Self {
            name,
            mileage,
            energy,
            generation,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Mileage(pub usize);

#[derive(PartialEq, Debug)]
pub struct Energy(pub String);

#[derive(PartialEq, Debug)]
pub struct Generation(pub usize);

#[derive(Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(transparent)]
pub struct EcologicalNote(pub f32);
