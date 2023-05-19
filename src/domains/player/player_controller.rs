use actix_web::{
    HttpResponse,
    post,
    web::{Data, Json},
};
use crate::domains::player::player_service::PlayersService;
use crate::repository::domains::players::dtos::create_player_dto::CreatePlayerDto;
use crate::repository::mongo_db::collections::players::players_mongodb::PlayersMongodb;

#[post("/player")]
pub async fn create_player(repository: Data<PlayersMongodb>, new_player: Json<CreatePlayerDto>) -> HttpResponse {
    let player_details = PlayersService::create_player(repository.get_ref(), new_player).await;
    match player_details {
        Ok(details) => HttpResponse::Ok().json(details),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
