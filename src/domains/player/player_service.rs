use actix_web::web::Json;
use crate::repository::domains::players::dtos::create_player_dto::CreatePlayerDto;
use crate::repository::domains::players::dtos::get_player_dto::GetPlayerDto;
use crate::repository::domains::players::player_repository::PlayersRepository;

pub struct PlayersService {}

impl PlayersService {
    pub async fn create_player(repository: &impl PlayersRepository, new_player_dto: Json<CreatePlayerDto>) -> Result<GetPlayerDto, String> {
        let new_player = CreatePlayerDto {
            name: new_player_dto.name.to_owned(),
            gold: new_player_dto.gold.to_owned(),
        };

        let player = repository
            .create_one(new_player)
            .await
            .ok()
            .expect("Error creating player");
        Ok(player)
    }
}
