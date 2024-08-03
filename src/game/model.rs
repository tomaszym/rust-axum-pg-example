use serde::{Deserialize, Serialize};
use ulid::Ulid;
use crate::core::ids::{GameId, PlayerId};

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub game_id: GameId,
    pub players: Vec<PlayerId>
}

