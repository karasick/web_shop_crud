use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePlayerDto {
    pub name: String,
    pub gold: f32,
}
