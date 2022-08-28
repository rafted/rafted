use crate::entity::{self, player::Player};

type World = i32;

#[derive(Debug, Clone)]
pub struct Server {
    players: Vec<Player<'static>>,
    worlds: Vec<World>,
}

impl Server {
    fn new() -> Server {
        return Server {
            players: vec![],
            worlds: vec![],
        };
    }
}
