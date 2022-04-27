use crate::beagle_math;
use crate::asset;

pub struct Renderable {
    pub renderable_meshes: Vec<RenderableMesh>
}

impl Renderable {
    pub fn from_model(model: &asset::mesh::Model) -> Self {
        let renderable_meshes: Vec<RenderableMesh> = model.meshes
            .iter()
            .map(|x| {
                let children = x.children.clone();
                let translation = x.translation;
                let scale = x.scale;
                let rotation = x.rotation;
                let vertex_positions: Vec<beagle_math::Vector3> = Renderable::expand_vertex_buffer_by_indices(&x.indices, &x.vertex_positions);

                RenderableMesh {
                    children,
                    translation,
                    scale,
                    rotation,
                    vertex_positions
                }
            }).collect();

        Renderable { 
            renderable_meshes 
        }
    }

    fn expand_vertex_buffer_by_indices<T: Copy>(indices: &Vec<u16>, vertex_buffer: &Vec<T>) -> Vec<T> {
        indices.iter().map(|x| vertex_buffer.get((*x) as usize).unwrap().clone()).collect()
    }
}

pub struct RenderableMesh {
    pub children: Vec<u16>,
    pub translation: beagle_math::Vector3,
    pub scale: beagle_math::Vector3,
    pub rotation: beagle_math::Quaternion,
    pub vertex_positions: Vec<beagle_math::Vector3>
}