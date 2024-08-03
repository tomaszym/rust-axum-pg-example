mod core;
mod player;
mod game;

use std::sync::{Arc, Mutex};
use axum::{routing::get, Router, Json};
use axum::extract::State;
use axum::http::StatusCode;
use tokio_postgres::{NoTls, Error};
use ulid::Ulid;
use crate::core::ids::GameId;
use crate::core::ids::PlayerId;
use crate::game::model::Game;
use crate::player::model::Player;
use crate::player::repo::{PgPlayerRepo, PlayerRepo};

#[tokio::main]
async fn main() -> Result<(), Error>{

    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=changeme", NoTls).await?;

    let client_ref = Arc::new(Mutex::new(client));

    let pg_player_repo = PgPlayerRepo { client: client_ref};
    let player_repo = Arc::new( pg_player_repo);

    let state = AppState {
        player_repo
    };

    let app = Router::new()
        .route("/game", get(get_game))
        .route("/player", get(get_player))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    Ok(axum::serve(listener, app).await.unwrap())
}

#[derive(Clone)]
pub struct AppState {
    pub player_repo: Arc<dyn PlayerRepo>
}

async fn get_game(
    State(state): State<AppState>
) -> Json<Game> {

    let alice = Player {
        player_id: PlayerId(Ulid::new())
    };
    let bob = Player {
        player_id: PlayerId(Ulid::new())
    };

    let players = vec![alice.player_id, bob.player_id];

    let game = Game {
        game_id: GameId(Ulid::new()),
        players: players,
    };

    Json(game)
}

async fn get_player(
    State(state): State<AppState>,
) -> Result<Json<Player>, StatusCode> {

    match state.player_repo.clone().get_player(PlayerId(Ulid::new())) {
        Some(player) => Ok(Json(player)),
        None => Err(StatusCode::NOT_FOUND),
    }

}