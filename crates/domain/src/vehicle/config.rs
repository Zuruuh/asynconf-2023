use schemars::JsonSchema;
use serde::Deserialize;

use crate::boundary::{Boundary, OptionalBoundary};

use super::EcologicalNote;

/// Model classes for configuration (the one defined in `data.json`)

#[derive(Deserialize, Debug, JsonSchema)]
pub struct VehicleConfig {
    pub name: String,
    pub average_weight: Boundary,
    pub ecological_note: EcologicalNote,
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct EnergyConfig {
    pub name: String,
    pub ecological_note: EcologicalNote,
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct MileageConfig {
    #[serde(flatten)]
    pub boundary: OptionalBoundary,
    pub ecological_note: EcologicalNote,
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct GenerationConfig {
    #[serde(flatten)]
    pub boundary: OptionalBoundary,
    pub ecological_note: EcologicalNote,
}
