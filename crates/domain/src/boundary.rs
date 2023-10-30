use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
pub struct Boundary {
    pub lower_boundary: usize,
    pub upper_boundary: usize,
}

#[derive(Deserialize, JsonSchema)]
pub struct OptionalBoundary {
    pub lower_boundary: Option<usize>,
    pub upper_boundary: Option<usize>,
}
