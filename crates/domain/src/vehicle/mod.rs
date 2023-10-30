use schemars::JsonSchema;
use serde::Deserialize;

pub mod config;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Mileage(pub usize);

#[derive(Debug)]
pub struct Energy(pub String);

#[derive(Debug)]
pub struct Generation(pub usize);

#[derive(Deserialize, Debug, JsonSchema)]
#[serde(transparent)]
pub struct EcologicalNote(pub f32);
