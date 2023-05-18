use mongodb::{
    results::InsertOneResult,
    bson::extjson::de::Error as MongodbBsonError,
};
use crate::repository::mongo_db::collections::players::player_repository::PlayerCollection;
use crate::repository::mongo_db::collections::players::player_schema::PlayerSchema;

pub struct PlayersService {
    col: PlayerCollection,
}

impl PlayersService {
    pub fn init(col: PlayerCollection) -> Self {
        PlayersService { col }
    }

    pub async fn create_player(&self, new_player: PlayerSchema) -> Result<InsertOneResult, MongodbBsonError> {
        let new_doc = PlayerSchema {
            _id: None,
            name: new_player.name,
            gold: new_player.gold,
        };
        let player = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating player");
        Ok(player)
    }
}
