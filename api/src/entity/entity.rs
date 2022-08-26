pub trait EntityType {
    fn get_id(&self) -> i32;
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
}

pub trait Entity<T>
where
    T: EntityType,
{
    fn get_world_id(&self) -> i32;
    fn get_body(&self) -> Option<EntityBody>;
    fn create_body(&mut self);
    fn get_entity_type() -> T;
}

#[derive(Clone, Copy, Debug)]
pub struct EntityBody {
    pub position: Position,
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
}
