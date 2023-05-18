use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use crate::domains::player::player_service::PlayersService;
use crate::repository::mongo_db::collections::players::player_repository::{PlayerCollection};
use crate::repository::mongo_db::collections::players::player_schema::PlayerSchema;

#[post("/player")]
pub async fn create_player(collection: Data<PlayerCollection>, new_player: Json<PlayerSchema>) -> HttpResponse {
    let player_service = PlayersService::init(collection.get_ref().to_owned());

    let data = PlayerSchema {
        _id: None,
        name: new_player.name.to_owned(),
        gold: new_player.gold.to_owned(),
    };
    let player_details = player_service.create_player(data).await;
    match player_details {
        Ok(player) => HttpResponse::Ok().json(player),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}