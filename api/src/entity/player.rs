use super::entity::{Body, Entity, Position, Messageable};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Player {
    entity_id: i32,
    unique_id: Uuid,
    position: Position,
}

impl Player {
    pub fn new(unique_id: Uuid) -> Self {
        // Implement data for location etc"

        return Self {
            entity_id: 0,
            position: Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                yaw: 0.0,
                pitch: 0.0,
                world: "normal".into(),
            },
            unique_id,
        };
    }

    pub fn get_unique_id(&self) -> Uuid {
        return self.unique_id;
    }
}

impl Messageable for Player {
    fn send_message(&self, msg: String) {
        println!("sending message: {}", msg);
    }
}

impl Entity for Player {
    fn get_entity_id(&self) -> i32 {
        return self.entity_id;
    }
}

impl Body for Player {
    fn teleport(&mut self, to: Position) {
        println!("{:?}", to)
    }

    fn get_position(&self) -> &Position {
        return &self.position;
    }
}
