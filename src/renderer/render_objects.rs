use crate::frustum::{Frustum, ObjectBound};

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum ShapeType {
    None = 0,
    Box = 1,
    Sphere = 2,
    Cylinder = 3,
    Swordman = 4,
    Gunman = 5,
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum MaterialType {
    Green = 0,
    Yellow = 1,
    White = 2,
    Black = 3,
    Checker = 4,
    Red = 5,
    Orange = 6,
}

#[derive(Debug, Copy, Clone)]
pub struct RenderQueueData {
    pub position: nalgebra::Vector3<f32>,
    _p1: [i32; 1],
    pub scale: nalgebra::Vector3<f32>,
    _p2: [i32; 1],
    pub rotation: nalgebra::Vector3<f32>,
    _p3: [i32; 1],
    pub shape_data1: nalgebra::Vector4<f32>,
    pub shape_data2: nalgebra::Vector4<f32>,
    pub shape_type_material_ids: (ShapeType, MaterialType, MaterialType, MaterialType),
}

impl RenderQueueData {
    pub fn new_none() -> Self {
        Self {
            position: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            scale: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            rotation: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            shape_type_material_ids: (
                ShapeType::None,
                MaterialType::Red,
                MaterialType::Red,
                MaterialType::Red,
            ),
            shape_data1: nalgebra::Vector4::new(0.0, 0.0, 0.0, 0.0),
            shape_data2: nalgebra::Vector4::new(0.0, 0.0, 0.0, 0.0),
            _p1: [0; 1],
            _p2: [0; 1],
            _p3: [0; 1],
        }
    }
}

pub struct RenderObjects {
    render_objects: Vec<(RenderQueueData, ObjectBound)>,
    render_objects_static: Vec<(RenderQueueData, ObjectBound)>,
}

impl RenderObjects {
    pub fn new() -> Self {
        Self {
            render_objects: Vec::new(),
            render_objects_static: Vec::new(),
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

    pub fn get_objects_and_active_len(
        &mut self,
        frustum: &Frustum,
    ) -> ([RenderQueueData; 70], usize) {
        let mut resulted_objects = [RenderQueueData::new_none(); 70];
        let mut index = 0;
        for (object, bound) in self.render_objects_static.iter() {
            if frustum.is_on_frustum(&object.position, bound) {
                resulted_objects[index] = *object;
                index += 1;
            }
        }
        for (object, bound) in self.render_objects.iter() {
            if frustum.is_on_frustum(&object.position, bound) {
                resulted_objects[index] = *object;
                index += 1;
            }
        }
        self.render_objects.clear();
        (resulted_objects, index)
    }
}
