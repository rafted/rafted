type Player = i64;
type World = i64;

#[derive(Debug, Clone)]
pub struct Server {
    players: Vec<Player>,
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
