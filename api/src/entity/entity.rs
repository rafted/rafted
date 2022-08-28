pub trait Entity: Send + Copy + Clone {
    fn get_entity_id(&self) -> i32;
}

pub trait Body: Send + Copy + Clone {
    fn get_position(&self) -> Position;
    fn teleport(&mut self, to: Position);
}

#[derive(Clone, Copy, Debug)]
pub struct Position<'a> {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub world: &'a str,
}
