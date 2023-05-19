use async_trait::async_trait;
use mongodb::{Collection, Database};
use crate::repository::domains::players::dtos::create_player_dto::CreatePlayerDto;
use crate::repository::domains::players::dtos::get_player_dto::GetPlayerDto;
use crate::repository::domains::players::player_repository::{PlayersRepository, REPOSITORY_NAME};
use crate::repository::mongo_db::collections::players::player_schema::PlayerSchema;

pub struct PlayersMongodb {
    col: Collection<PlayerSchema>,
}

impl PlayersMongodb {
    pub async fn init(db: Database) -> Self {
        println!("Connecting to {} collection", REPOSITORY_NAME);
        let collection = db.collection::<PlayerSchema>(REPOSITORY_NAME);
        println!("Connected to {}", collection.name());

        PlayersMongodb { col: collection }
    }
}

#[async_trait]
impl PlayersRepository for PlayersMongodb {
    async fn create_one(&self, new_player: CreatePlayerDto) -> Result<GetPlayerDto, String> {
        let new_doc = PlayerSchema {
            _id: None,
            name: new_player.name.clone(),
            gold: new_player.gold.clone(),
        };
        let player = self.col.insert_one(new_doc, None).await;
        match player {
            Ok(value) => {
                println!("{:?}", value);
                let inserted_doc = GetPlayerDto {
                    id: value.inserted_id.as_object_id().unwrap().to_string(),
                    name: new_player.name.to_owned(),
                    gold: new_player.gold.to_owned(),
                };
                Ok(inserted_doc)
            },
            Err(err) => {
                println!("{:?}", err);
                Err(err.to_string())
            }
        }
    }
}
