pub trait Entity: Send {
    fn get_entity_id(&self) -> i32;
}

impl PartialEq for dyn Entity + '_ {
    fn eq(&self, other: &Self) -> bool {
        return self.get_entity_id() == other.get_entity_id();
    }
}

pub trait Body: Send {
    fn get_position(&self) -> &Position;
    fn teleport(&mut self, to: Position);
}

pub trait Recipient: Send {
    fn send_message(&self, msg: String);
}

#[derive(Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub world: String,
}
