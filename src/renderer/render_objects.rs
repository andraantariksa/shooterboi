use crate::frustum::{Frustum, ObjectBound};
use nalgebra::{Matrix4, Vector3, Vector4};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum ShapeType {
    None = 0,
    Box = 1,
    Sphere = 2,
    Cylinder = 3,
    Swordman = 4,
    Gunman = 5,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum MaterialType {
    Green = 0,
    Yellow = 1,
    White = 2,
    Black = 3,
    // Checker = 4,
    Red = 5,
    Orange = 6,
    Crate = 7,
    Pebbles = 8,
    CobblestonePaving = 9,
    Container = 10,
    Target = 11,
    Grass = 12,
    StoneWall = 13,
    Building = 14,
    TargetDimmed = 15,
    TreeBark = 16,
    Leaves = 17,
    RGBANoiseMedium = 18,
    GrayNoiseSmall = 19,
    Asphalt = 20,
}

#[derive(Debug, Clone, Copy)]
pub struct RenderQueueData {
    pub position: Vector3<f32>,
    pub scale: f32,
    pub rotation: Matrix4<f32>,
    pub shape_data1: Vector4<f32>,
    pub shape_data2: Vector4<f32>,
    pub shape_type_material_ids: (ShapeType, MaterialType, MaterialType, MaterialType),
}

impl RenderQueueData {
    pub fn new_none() -> Self {
        Self {
            position: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            scale: 1.0,
            rotation: Matrix4::identity(),
            shape_type_material_ids: (
                ShapeType::None,
                MaterialType::Red,
                MaterialType::Red,
                MaterialType::Red,
            ),
            shape_data1: nalgebra::Vector4::new(0.0, 0.0, 0.0, 0.0),
            shape_data2: nalgebra::Vector4::new(0.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn get_bounding_sphere_radius(&self) -> ObjectBound {
        match self.shape_type_material_ids.0 {
            ShapeType::None => ObjectBound::None,
            ShapeType::Box => {
                let a = self.shape_data1.xyz();
                ObjectBound::Sphere(a.component_mul(&a).magnitude())
            }
            ShapeType::Sphere => ObjectBound::Sphere(self.shape_data1.x),
            _ => unreachable!(),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub const QUEUE_SIZE: usize = 130;
#[cfg(target_arch = "wasm32")]
pub const QUEUE_SIZE: usize = 60;

pub struct RenderObjects {
    render_objects: Vec<(RenderQueueData, ObjectBound)>,
    render_objects_static: Vec<(RenderQueueData, ObjectBound)>,
    resulted_objects: Vec<RenderQueueData>,
}

impl RenderObjects {
    pub fn new() -> Self {
        Self {
            render_objects: Vec::new(),
            render_objects_static: Vec::new(),
            resulted_objects: vec![RenderQueueData::new_none(); QUEUE_SIZE],
        }
    }

    pub fn get_mut(&mut self, index: usize) -> &mut (RenderQueueData, ObjectBound) {
        self.render_objects.get_mut(index).unwrap()
    }

    pub fn get_mut_static(&mut self, index: usize) -> &mut (RenderQueueData, ObjectBound) {
        self.render_objects_static.get_mut(index).unwrap()
    }

    pub fn next(&mut self) -> &mut (RenderQueueData, ObjectBound) {
        let length = self.render_objects.len();
        self.render_objects
            .push((RenderQueueData::new_none(), ObjectBound::None));
        self.render_objects.get_mut(length).unwrap()
    }

    pub fn next_static(&mut self) -> &mut (RenderQueueData, ObjectBound) {
        let length = self.render_objects_static.len();
        self.render_objects_static
            .push((RenderQueueData::new_none(), ObjectBound::None));
        self.render_objects_static.get_mut(length).unwrap()
    }

    pub fn clear(&mut self) {
        self.render_objects.clear();
        self.render_objects_static.clear();
    }

    pub fn get_objects_and_active_len(&mut self, frustum: &Frustum) -> Vec<RenderQueueData> {
        for object in self.resulted_objects.iter_mut() {
            *object = RenderQueueData::new_none();
        }

        let mut index = 0;
        for (object, bound) in self.render_objects_static.iter() {
            if index >= QUEUE_SIZE {
                break;
            }
            if frustum.is_on_frustum(&object.position, bound) {
                self.resulted_objects[index] = *object;
                index += 1;
            }
        }
        for (object, bound) in self.render_objects.iter() {
            if index >= QUEUE_SIZE {
                break;
            }
            if frustum.is_on_frustum(&object.position, bound) {
                self.resulted_objects[index] = *object;
                index += 1;
            }
        }
        self.render_objects.clear();
        self.resulted_objects.clone()
    }
}
