use crate::renderer::render_objects::MaterialType;

pub mod gunman;
pub mod swordman;

pub const HITTED_MATERIAL_DURATION: f32 = 0.1;

pub trait HasMaterial {
    fn get_material(&self) -> MaterialType;
}
