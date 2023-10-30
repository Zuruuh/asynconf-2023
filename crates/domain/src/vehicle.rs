use schemars::JsonSchema;
use serde::Deserialize;

use crate::boundary::{Boundary, OptionalBoundary};

#[derive(Deserialize, JsonSchema)]
pub struct Vehicle {
    pub name: String,
    pub average_weight: Boundary,
    pub ecological_note: EcologicalNote,
}

#[derive(Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct EcologicalNote(#[validate(range(min = 1, max = 10))] pub f32);

#[derive(Deserialize, JsonSchema)]
pub struct Energy {
    pub name: String,
    pub ecological_note: EcologicalNote,
}

#[derive(Deserialize, JsonSchema)]
pub struct Mileage {
    #[serde(flatten)]
    pub boundary: OptionalBoundary,
    pub ecological_note: EcologicalNote,
}

#[derive(Deserialize, JsonSchema)]
pub struct Generation {
    #[serde(flatten)]
    pub boundary: Boundary,
    pub ecological_note: EcologicalNote,
}
