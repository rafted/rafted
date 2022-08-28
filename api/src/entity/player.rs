use super::entity::{Body, Entity, Position};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Player<'a> {
    entity_id: i32,
    unique_id: Uuid,
    position: Position<'a>,
}

impl<'a> Player<'a> {
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
                world: "normal",
            },
            unique_id,
        };
    }

    pub fn send_message(&self, message: &str) {
        println!("sending message: {}", message);
        todo!("implement")
    }

    pub fn get_unique_id(&self) -> Uuid {
        return self.unique_id;
    }
}

impl<'a> Entity for Player<'a> {
    fn get_entity_id(&self) -> i32 {
        return self.entity_id;
    }
}

impl<'a> Body for Player<'a> {
    fn teleport(&mut self, to: Position) {
        println!("{:?}", to)
    }

    fn get_position(&self) -> Position {
        return self.position;
    }
}
