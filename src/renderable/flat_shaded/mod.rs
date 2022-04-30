use crate::beagle_math;
use crate::asset;

pub struct Renderable {
    pub renderable_meshes: Vec<RenderableMesh>
}

impl Renderable {
    pub fn from_model(model: &asset::mesh::Model) -> Self {
        let renderable_meshes: Vec<RenderableMesh> = model.meshes
            .iter()
            .map(|mesh| {
                let children = mesh.children.clone();
                let translation = mesh.translation;
                let scale = mesh.scale;
                let rotation = mesh.rotation;
                let material = Material {
                    diffuse_color: mesh.material.diffuse_color,
                    ambient_color: mesh.material.ambient_color,
                    specular_color: mesh.material.specular_color,
                    shininess_factor: mesh.material.shininess_factor
                };
                let vertex_positions: Vec<beagle_math::Vector3> = Renderable::expand_vertex_buffer_by_indices(&mesh.indices, &mesh.vertex_positions);

                RenderableMesh {
                    children,
                    material,
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
    pub material: Material,
    pub translation: beagle_math::Vector3,
    pub scale: beagle_math::Vector3,
    pub rotation: beagle_math::Quaternion,
    pub vertex_positions: Vec<beagle_math::Vector3>
}

pub struct Material {
    pub diffuse_color: beagle_math::Vector3,
    pub ambient_color: beagle_math::Vector3,
    pub specular_color: beagle_math::Vector3,
    pub shininess_factor: f32
}