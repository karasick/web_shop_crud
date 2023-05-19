mod domains;
mod repository;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use dotenv::from_filename;
use crate::domains::player::player_controller::create_player;
use crate::repository::mongo_db::collections::players::players_mongodb::PlayersMongodb;
use crate::repository::mongo_db::connection_factory::get_mongodb_connection;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // for env vars in dev
    from_filename(".env.dev").ok();
    env_logger::init();

    let db = get_mongodb_connection().await.unwrap();

    let players_db = PlayersMongodb::init(db.clone()).await;
    let players_data = Data::new(players_db);

    HttpServer::new(move || {
        App::new()
            .app_data(players_data.clone())
            .service(hello)
            .service(create_player)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
