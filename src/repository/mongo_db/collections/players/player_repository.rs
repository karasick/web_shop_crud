use mongodb::{Collection, Database};
use crate::repository::mongo_db::collections::players::player_schema::PlayerSchema;

const COLLECTION_NAME: &str = "players";
pub type PlayerCollection = Collection<PlayerSchema>;

pub struct PlayersRepository {
    pub col: PlayerCollection,
}

impl PlayersRepository {
    pub async fn init(db: Database) -> Self {
        println!("Connecting to {COLLECTION_NAME} collection");
        let collection = db.collection::<PlayerSchema>(COLLECTION_NAME);
        println!("Connected to {}", collection.name());

        PlayersRepository { col: collection }
    }
}
