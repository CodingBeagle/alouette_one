use std::{ffi::c_void, mem};

use windows::{
    Win32::{
        System::*,
        UI::WindowsAndMessaging::*,
        Foundation::*,
        Graphics::Direct3D::*,
        Graphics::Direct3D11::*,
        Graphics::Dxgi::*,
        Graphics::Dxgi::Common::*,
    }, core::{Interface}  
};


use crate::beagle_math;
use crate::asset;
use crate::dx;

enum Usage {
    GpuReadWrite = D3D11_USAGE_DEFAULT as isize,
    GpuReadCpuWrite = D3D11_USAGE_DYNAMIC as isize
}

enum BufferType {
    Vertex = D3D11_BIND_VERTEX_BUFFER as isize,
    Index = D3D11_BIND_INDEX_BUFFER as isize,
    Shader = D3D11_BIND_CONSTANT_BUFFER as isize
}

enum CpuAccess {
    None = 0,
    Read = D3D11_CPU_ACCESS_READ as isize,
    Write = D3D11_CPU_ACCESS_WRITE as isize
}

pub struct Renderable {
    pub renderables: Vec<RenderableMesh>
}

impl Renderable {
    pub fn from_render_data(render_data: RenderData) -> Renderable {
        let mut renderables: Vec<RenderableMesh> = vec!();

        for renderable_mesh_data in render_data.renderable_mesh_data {
            renderables.push(
                RenderableMesh {
                    vertex_buffer: Renderable::create_buffer::<beagle_math::Vector3>(BufferType::Vertex, Usage::GpuReadWrite, CpuAccess::None, &renderable_mesh_data.vertex_positions),
                    normals_buffer: Renderable::create_buffer::<beagle_math::Vector3>(BufferType::Vertex, Usage::GpuReadWrite, CpuAccess::None, &renderable_mesh_data.vertex_normals),
                    debug_vertex_normals_buffer: Renderable::create_buffer::<beagle_math::Vector3>(BufferType::Vertex, Usage::GpuReadWrite, CpuAccess::None, &renderable_mesh_data.debug_vertex_normals),
                    renderable_mesh_data: renderable_mesh_data
                }
            )
        }

        Renderable {
            renderables
        }
    }

    fn create_buffer<T>(bufferType: BufferType, usage: Usage, cpu_access: CpuAccess, initial_data: &[T]) -> ID3D11Buffer {
        unsafe {
            let buffer_description = D3D11_BUFFER_DESC {
                ByteWidth: (mem::size_of::<T>() * initial_data.len()) as u32,
                Usage: usage as i32,
                BindFlags: bufferType as u32,
                CPUAccessFlags: cpu_access as u32,
                MiscFlags: 0,
                StructureByteStride: 0
            };
        
            let buffer_subresource = D3D11_SUBRESOURCE_DATA {
                pSysMem: initial_data.as_ptr() as *mut c_void,
                SysMemPitch: 0,
                SysMemSlicePitch: 0
            };
        
            let dx_device = &dx::DX.as_ref().unwrap().device;
    
            match dx_device.CreateBuffer(&buffer_description, &buffer_subresource) {
                Ok(id) => id,
                Err(err) => panic!("Failed to create buffer: {}", err)
            }
        }
    }
}

pub struct RenderableMesh {
    pub renderable_mesh_data: RenderableMeshData,
    pub vertex_buffer: ID3D11Buffer,
    pub normals_buffer: ID3D11Buffer,
    pub debug_vertex_normals_buffer: ID3D11Buffer
}

pub struct RenderData {
    pub renderable_mesh_data: Vec<RenderableMeshData>
}

impl RenderData {
    pub fn from_model(model: &asset::mesh::Model) -> Self {
        let renderable_meshes: Vec<RenderableMeshData> = model.meshes
            .iter()
            .map(|mesh| {
                let name = mesh.name.clone();
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
                let vertex_positions= RenderData::expand_vertex_buffer_by_indices(&mesh.indices, &mesh.vertex_positions);
                let vertex_normals = RenderData::calculate_vertex_normals(&vertex_positions);
                let debug_vertex_normals = RenderData::create_vertex_normal_debug_buffer(&vertex_positions, &vertex_normals);

                RenderableMeshData {
                    name,
                    children,
                    material,
                    translation,
                    scale,
                    rotation,
                    vertex_positions,
                    vertex_normals,
                    debug_vertex_normals
                }
            }).collect();

        RenderData { 
            renderable_mesh_data: renderable_meshes 
        }
    }

    fn expand_vertex_buffer_by_indices<T: Copy>(indices: &Vec<u16>, vertex_buffer: &Vec<T>) -> Vec<T> {
        indices.iter().map(|x| vertex_buffer.get((*x) as usize).unwrap().clone()).collect()
    }

    // This buffer is a list of point pairs, each pair being the position of a vertex coupled with a point representing the end of its vertex normal.
    // This buffer can be used for debug rendering of a renderable's vertex normals.
    // This function expects a list of vertex positions and a list of vertex normals, with the vertex normal associated to a 
    // vertex position at the SAME index as the vertex position.
    fn create_vertex_normal_debug_buffer(vertex_positions: &Vec<beagle_math::Vector3>, vertex_normals: &Vec<beagle_math::Vector3>) -> Vec<beagle_math::Vector3> {
        let mut result: Vec<beagle_math::Vector3> = vec!();

        for (index, vertex_normal) in vertex_normals.iter().enumerate() {
            // When working with vectors, it's important to realize if you're working on something representing a position or a direction.
            // Because it makes no sense to normalize a vector representing a position. That will screw with your positions, obviously.
            let scaled_vertex_normal = vertex_normal.mul(0.2 / vertex_normal.length());

            let vertex_normal_end_relative_to_vertex_position = beagle_math::Vector3::new(
                vertex_positions[index].x + scaled_vertex_normal.x,
                vertex_positions[index].y + scaled_vertex_normal.y,
                vertex_positions[index].z + scaled_vertex_normal.z,
            );

            result.push(vertex_positions[index]);
            result.push(vertex_normal_end_relative_to_vertex_position);
        }

        result
    }

    /*
        This method assumes a list of vertices, with 3 vertices making up a triangle.
        Each 3 vertice making up one triangle will get the same vertex normal, representing perpedicularity to the same surface.

        Trying out this technique: https://computergraphics.stackexchange.com/questions/4031/programmatically-generating-vertex-normals
    */
    fn calculate_vertex_normals(vertex_positions: &Vec<beagle_math::Vector3>) -> Vec<beagle_math::Vector3> {
        if vertex_positions.len() % 3 != 0 {
            panic!("The list of vertex positions is not divisible by 3, which is required as this method assumes a primitive topology of triangles.")
        }

        let mut vertex_normals: Vec<beagle_math::Vector3> = vec!();

        for vertex_position_index in (0..vertex_positions.len()).step_by(3) {
            let vert1 = vertex_positions[vertex_position_index];
            let vert2 = vertex_positions[vertex_position_index + 1];
            let vert3 = vertex_positions[vertex_position_index + 2];

            let edge1 = beagle_math::Vector3::new(
                vert2.x - vert1.x,
                vert2.y - vert1.y,
                vert2.z - vert1.z,
            );

            let edge2 = beagle_math::Vector3::new(
                vert3.x - vert1.x,
                vert3.y - vert1.y,
                vert3.z - vert1.z,
            );

            let mut vertex_normal = edge1.cross(&edge2).normalized();

            vertex_normals.push(vertex_normal);
            vertex_normals.push(vertex_normal);
            vertex_normals.push(vertex_normal);
        }

        vertex_normals
    }
}

pub struct RenderableMeshData {
    pub name: String,
    pub children: Vec<u16>,
    pub material: Material,
    pub translation: beagle_math::Vector3,
    pub scale: beagle_math::Vector3,
    pub rotation: beagle_math::Quaternion,
    pub vertex_positions: Vec<beagle_math::Vector3>,
    pub vertex_normals: Vec<beagle_math::Vector3>,
    pub debug_vertex_normals: Vec<beagle_math::Vector3>
}

pub struct Material {
    pub diffuse_color: beagle_math::Vector3,
    pub ambient_color: beagle_math::Vector3,
    pub specular_color: beagle_math::Vector3,
    pub shininess_factor: f32
}