use serde::{Deserialize, Serialize};
use ulid::Ulid;
use crate::core::ids::PlayerId;

#[derive(Serialize, Deserialize)]
pub struct Player{
    pub player_id: PlayerId,
}