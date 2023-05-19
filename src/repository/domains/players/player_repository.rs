use async_trait::async_trait;
use crate::repository::domains::players::dtos::create_player_dto::CreatePlayerDto;
use crate::repository::domains::players::dtos::get_player_dto::GetPlayerDto;

pub const REPOSITORY_NAME: &str = "players";

#[async_trait]
pub trait PlayersRepository {
    async fn create_one(&self, new_player: CreatePlayerDto) -> Result<GetPlayerDto, String>;
}
