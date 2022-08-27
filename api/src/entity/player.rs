use super::entity::{Entity, EntityBody, EntityType, Position};
use uuid::Uuid;

struct PlayerType;

// maybe use macros for entity types
impl EntityType for PlayerType {
    fn get_height(&self) -> f32 {
        return 1.8;
    }

    fn get_width(&self) -> f32 {
        return 0.6;
    }

    fn get_id(&self) -> i32 {
        return 116;
    }
}

struct Player {
    entity_id: i32,
    unique_id: Uuid,
    body: Option<EntityBody>,
}

impl Player {
    fn is_online(&self) -> bool {
        return self.body.is_some();
    }

    fn send_message(&self) {
        todo!("implement")
    }
}

impl Entity<PlayerType> for Player {
    fn get_world_id(&self) -> i32 {
        return self.entity_id;
    }

    fn get_body(&self) -> Option<EntityBody> {
        return self.body;
    }

    fn create_body(&mut self) {
        self.body = Some(EntityBody {
            position: Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                pitch: 0.0,
                yaw: 0.0,
            },
        });
    }

    fn get_entity_type() -> PlayerType {
        return PlayerType;
    }
}
