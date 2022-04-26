use crate::beagle_math;
use crate::gltf2;
use crate::shared;

#[derive(Default)]
pub struct Model {
    pub meshes: Vec<Mesh>
}

#[derive(Default)]
pub struct Mesh {
    pub name: String,
    pub children: Vec<u16>,
    pub translation: beagle_math::Vector3,
    pub scale: beagle_math::Vector3,
    pub rotation: beagle_math::Quaternion,
    pub vertex_positions: Vec<beagle_math::Vector3>,
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
    let mut meshes : Vec<Mesh> = vec!();

    for node in &gltf_file.nodes {
        let translation = beagle_math::Vector3::from_array(&node.translation);
        let scale = beagle_math::Vector3::from_array(&node.scale);
        let rotation = beagle_math::Quaternion::from_array(&node.rotation);
        let child_meshes : Vec<u16> = node.children.iter().map(|x| *x as u16).collect();

        let root_mesh_index = node.mesh as usize;
        let root_mesh = gltf_file.meshes.get(root_mesh_index).unwrap();

        let mesh_name = root_mesh.name.clone();

        let mut new_mesh = Mesh::default();
        new_mesh.name = mesh_name;
        new_mesh.translation = translation;
        new_mesh.scale = scale;
        new_mesh.rotation = rotation;
        new_mesh.children = child_meshes;
        new_mesh.vertex_positions = get_buffer_data_for_acessor::<beagle_math::Vector3>(gltf_file, root_mesh.primitives.first().unwrap().attributes.position as usize);

        meshes.push(new_mesh);
    }

    Model { meshes }
}

fn get_buffer_data_for_acessor<T: shared::FromBinary + Sized>(gltf_file: &gltf2::File, accessor_index: usize) -> Vec<T> {
    let buffer_view_index = gltf_file.accessors[accessor_index].buffer_view as usize;
    let buffer_view = &gltf_file.buffer_views[buffer_view_index];
    
    let buffer = &gltf_file.buffers[buffer_view.buffer as usize];

    let data_uri = &buffer.uri;
    if !data_uri.starts_with("data:application/octet-stream;base64") {
        panic!("Unsupported data URI encountered: {}", data_uri);
    }

    let data_in_base64 = data_uri.split_once(",").unwrap().1;
    let binary_data = base64::decode(data_in_base64).unwrap();

    let start_index = buffer_view.byte_offset as usize;
    let end_index = (buffer_view.byte_offset+buffer_view.byte_length) as usize;

    T::from_binary_collection(&binary_data[start_index..end_index])
}