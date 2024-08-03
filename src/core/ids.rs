use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Serialize, Deserialize)]
pub struct GameId(pub Ulid);
#[derive(Serialize, Deserialize)]
pub struct PlayerId(pub Ulid);