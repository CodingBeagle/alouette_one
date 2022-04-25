use crate::beagle_math;
use crate::gltf2;

#[derive(Default)]
pub struct Model {
    meshes: Vec<Mesh>
}

#[derive(Default)]
pub struct Mesh {
    pub name: String,
    pub children: Vec<u16>,
    pub translation: beagle_math::Vector3,
    pub scale: beagle_math::Vector3,
    pub rotation: beagle_math::Quaternion,
    pub vertex_positions: Vec<beagle_math::Vector3>,
    pub vertex_normals: Vec<beagle_math::Vector3>,
    pub material: Material
}

#[derive(Default)]
pub struct Material {
    pub diffuse_color: beagle_math::Vector3,
    pub ambient_color: beagle_math::Vector3,
    pub specular_color: beagle_math::Vector3,
    pub shininess_factor: f32
}

pub fn parse_model(gltf_file: &gltf2::File) -> Model {
    for node in &gltf_file.nodes {
        let translation = beagle_math::Vector3::new(
            node.translation[0],
            node.translation[1],
            node.translation[2],
        );

        let scale = beagle_math::Vector3::new(
            node.scale[0],
            node.scale[1],
            node.scale[2],
        );

        let rotation = beagle_math::Quaternion::new(
            node.rotation[3],
            node.rotation[0],
            node.rotation[1],
            node.rotation[2],
        );

        let root_mesh_index = node.mesh as usize;
        let root_mesh = gltf_file.meshes.get(root_mesh_index).unwrap();

        
    }

    unimplemented!()
}