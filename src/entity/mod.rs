use crate::renderer::render_objects::MaterialType;

pub mod enemy;

pub mod target;

pub struct Wall;
pub struct Crate;
pub struct Container;

pub trait HasMaterial {
    fn get_material(&self) -> MaterialType;
}
