use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPlayerDto {
    pub id: String,
    pub name: String,
    pub gold: f32,
}
