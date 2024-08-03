use std::sync::{Arc, Mutex};
use tokio_postgres::{Client, Connection};
use crate::core::ids::PlayerId;
use crate::player::model::Player;

pub trait PlayerRepo: Send + Sync {
    fn get_player(&self, id: PlayerId) -> Option<Player>;
}

pub struct PgPlayerRepo {
  pub  client: Arc<Mutex<Client>>
}

impl PlayerRepo for PgPlayerRepo {
    fn get_player(&self, id: PlayerId) -> Option<Player> {
        // self.client.lock().unwrap().query_opt("select * from players where id = $1")
        None
    }
}