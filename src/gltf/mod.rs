use std::{fs, ops::Div};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::cell::{RefCell, Ref};

extern crate base64;

use byteorder::{LittleEndian, ByteOrder};

use std::mem::{size_of};

use crate::beagle_math;

// TODO:
/*
    Perhaps I should split up the actual model that is read from JSON and the 
    logic that uses it to create importable data into two separate things.

    Combining model and logic to interpret and import it creates issues with mutating the structure itself
    during load time, that might become cleaner code by separating those two "concerns".
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GLTF {
    meshes: Vec<Mesh>,
    accessors: Vec<Accessor>,
    buffer_views: Vec<BufferView>,
    buffers: Vec<Buffer>
}

pub trait BinaryDecode {
    fn decode_binary(bytes: &[u8]) -> Self;
}

impl BinaryDecode for u16 {
    fn decode_binary(bytes: &[u8]) -> Self {
        LittleEndian::read_u16(bytes)
    }
}

pub trait Max {
    fn max() -> f32;
}

impl Max for u16 {
    fn max() -> f32 {
        u16::MAX as f32
    }
}

impl GLTF {
    pub fn new(gltf_path: PathBuf) -> GLTF {
        let gltf_file_content = fs::read_to_string(gltf_path).unwrap();
        serde_json::from_str(&gltf_file_content).unwrap()
    }

    /*
        This method assumes a list of vertices, with 3 vertices making up a triangle.
        Each 3 vertice making up one triangle will get the same vertex normal, representing perpedicularity to the same surface.

        Trying out this technique: https://computergraphics.stackexchange.com/questions/4031/programmatically-generating-vertex-normals
    */
    pub fn calculate_vertex_normals(vertex_positions: &Vec<beagle_math::Vector3>) -> Vec<beagle_math::Vector3> {
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

            let vertex_normal = edge1.cross(&edge2).normalized();

            vertex_normals.push(vertex_normal);
            vertex_normals.push(vertex_normal);
            vertex_normals.push(vertex_normal);
        }

        vertex_normals
    }

    pub fn expand_by_indices<T: Copy>(indices: &Vec<u16>, buffer_to_expand: &Vec<T>) -> Vec<T> {
        let mut the_result: Vec<T> = vec!();

        for i in indices {
            let corresponding_vertex_position = buffer_to_expand[(*i) as usize];
            the_result.push(corresponding_vertex_position);
        }

        the_result
    }

    pub fn decode_binary_to_scalar(binary_scalar_data: &[u8]) -> Vec<u16> {
        let size_of_u16_in_bytes = size_of::<u16>();

        if binary_scalar_data.len() % size_of_u16_in_bytes != 0 {
            panic!("Binary scalar data is not divisible by size of unsigned short in bytes, which is {}", size_of_u16_in_bytes);
        }

        let mut result: Vec<u16> = vec!();

        for i in (0..(binary_scalar_data.len())).step_by(size_of_u16_in_bytes) {
            let the_scalar = LittleEndian::read_u16(&binary_scalar_data[i..(i + size_of_u16_in_bytes)]);
            result.push(the_scalar);
        }

        result
    }

    pub fn decode_binary_to_vector3(binary_vector_data: &[u8]) -> Vec<beagle_math::Vector3> {
        let size_of_f32 = size_of::<f32>();
        let size_of_vector_in_bytes = size_of_f32 * 3;

        if binary_vector_data.len() % size_of_vector_in_bytes != 0 {
            panic!("Binary vector data is not divisible by size of a vector in bytes, which is {}", size_of_vector_in_bytes);
        }

        let mut result: Vec<beagle_math::Vector3> = vec!();

        for i in (0..(binary_vector_data.len())).step_by(size_of_vector_in_bytes) {
            let mut vector_elements: Vec<f32> = vec!();

            for x in (0..size_of_vector_in_bytes).step_by(size_of_f32) {
                let mut slice_start_offset = i + x;
                let mut slice_end_offset = slice_start_offset + size_of_f32;

                vector_elements.push(LittleEndian::read_f32(&binary_vector_data[slice_start_offset..slice_end_offset]));
            }

            result.push(beagle_math::Vector3::new(vector_elements[0], vector_elements[1], vector_elements[2]));
        }
 
        result
    }

    pub fn decode_binary_to_vector4<T: BinaryDecode + Into<f32> + Copy + Max>(binary_vector_data: &[u8]) -> Vec<beagle_math::Vector4> {
        let size_of_type = size_of::<T>();
        let size_of_vector_in_bytes = size_of_type * 4;

        if binary_vector_data.len() % size_of_vector_in_bytes != 0 {
            panic!("Binary vector data is not divisible by size of a vector in bytes, which is {}", size_of_vector_in_bytes);
        }

        let mut result: Vec<beagle_math::Vector4> = vec!();

        for i in (0..(binary_vector_data.len())).step_by(size_of_vector_in_bytes) {
            let mut vector_elements: Vec<T> = vec!();

            for x in (0..size_of_vector_in_bytes).step_by(size_of_type) {
                let mut slice_start_offset = i + x;
                let mut slice_end_offset = slice_start_offset + size_of_type;

                let decoded = T::decode_binary(&binary_vector_data[slice_start_offset..slice_end_offset]);

                vector_elements.push(decoded);
            }

            let x: f32 = T::into(vector_elements[0]) / T::max();
            let y: f32 = T::into(vector_elements[1]) / T::max();
            let z: f32 = T::into(vector_elements[2]) / T::max();
            let w: f32 = T::into(vector_elements[3]) / T::max();

            result.push(beagle_math::Vector4::new(x, y, z, w));
        }
 
        result
    }

    pub fn load_meshes(&self) -> Vec<LoadedMesh> {
        let mut loaded_meshes: Vec<LoadedMesh> = vec!();

        for mesh in &self.meshes {
            let mut loaded_primitives: Vec<LoadedPrimitive> = vec!();

            for primitive in &mesh.primitives {
                loaded_primitives.push(LoadedPrimitive {
                    vertex_positions: self.get_buffer_data_for_accessor(primitive.attributes.position as i32),
                    vertex_indices: self.get_buffer_data_for_accessor(primitive.indices as i32),
                    vertex_colors: Some(self.get_buffer_data_for_accessor(primitive.attributes.color_0 as i32)),
                    vertex_normals: Some(self.get_buffer_data_for_accessor(primitive.attributes.normal as i32))
                });
            }

            loaded_meshes.push(LoadedMesh {
                loaded_primitives
            });
        }

        loaded_meshes
    }

    fn get_buffer_data_for_accessor(&self, accessor_index: i32) -> LoadedBuffer {
        let accessor = &self.accessors[accessor_index as usize];

        let buffer_component = match accessor.element_type.as_str() {
            "VEC3" => BufferComponent::Vec3,
            "VEC4" => BufferComponent::Vec4,
            "SCALAR" => BufferComponent::Scalar,
            _ => panic!("Unsupported element type: {}", accessor.element_type)
        };

        let buffer_component_type = match accessor.component_type {
            5123 => if accessor.normalized { BufferComponentType::UnsignedShortNormalized } else { BufferComponentType::UnsignedShort },
            5126 => BufferComponentType::Float32,
            _ => panic!("Unsupported component type {}", accessor.component_type)
        };

        let buffer_view = &self.buffer_views[accessor.buffer_view as usize];
        let buffer = &self.buffers[buffer_view.buffer as usize];

        // TODO: Instead of copying buffer data, perhaps I should just have my
        // "LoadedBuffer" contain a reference to the underlying data returned from buffer.get_data.
        // Meaning, the lifetime of the underlying buffer data is attached to the GLTF model struct, which might make sense.
        let mut buffer_data: Vec<u8> = vec![0; buffer_view.byte_length as usize];
        buffer_data.copy_from_slice(&buffer.get_data()[buffer_view.byte_offset as usize..(buffer_view.byte_offset + buffer_view.byte_length) as usize]);

        LoadedBuffer {
            element_count: accessor.count,
            buffer_data,
            buffer_component,
            buffer_component_type
        }
    }
}

/*
    An Accessor defines a method for retrieving data as typed arrays
    from within a "Buffer View".

    The Accessor will specify things such as the component type, data type,
    the number of elements, etc...
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Accessor {
    // A reference index to the buffer view containing the corresponding data
    buffer_view: u32,
    // The data type of each individual value (component)
    // 5123 = Unsigned Short, 16 bits, 2 bytes
    // 5126 = float, 32 bits, 4 bytes
    component_type: u32,
    // Count is the number of elements in the buffer
    count: u32,
    // The normalized bool indicates whether the value has to be normalized before use.
    // That is, if the integer value has to be divided by its types MAX value before being used, in order to give a number between
    // [0, 1] for unsigned integer types, and [-1, 1] for signed integer types.
    #[serde(default)]
    normalized: bool,
    // The type of element that components are described as.
    // VEC3 = 3 components
    #[serde(rename = "type")]
    element_type: String
}

/*
    A "Buffer View" represents a contiguous segment of data
    in a buffer. You can have multiple buffer views into the same
    underlying buffer.
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BufferView {
    // A reference index to an underlying buffer.
    buffer: u32,
    // The amount of bytes in the buffer that this view cares about
    byte_length: u32,
    // The start offset in bytes for this buffer view.
    byte_offset: u32
}

// TODO: Using a variant of Interior Mutability pattern to mutate decoded_buffer
// Even though it's behind an immutable reference
// TODO: Perhaps it would be better design to have the entire GLTF struct immutable.
// Instead of turning to Interior Mutability. You'd simply just return the decoded buffer with no care for caching it.
// Instead, the loaded model struct that is served back when calling "load_meshes" will have cached copies.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Buffer {
    byte_length: u32,
    uri: String,
    #[serde(skip)]
    decoded_buffer: RefCell<Vec<u8>>
}

impl Buffer {
    pub fn get_data(&self) -> Ref<Vec<u8>> {
        if self.decoded_buffer.borrow().is_empty() {
            self.decoded_buffer.replace(self.decode_base64_data_uri(&self.uri));
        }

        self.decoded_buffer.borrow()
    }

    fn decode_base64_data_uri(&self, data_uri: &str) -> Vec<u8> {
        // https://en.wikipedia.org/wiki/Data_URI_scheme
        // data:[<media type>][;base64],<data>
        // TODO:
        //   Currently I'm very naive about my data URI parsing.
        //   Basically I only accept the strict starting format of "data:application/octet-stream;base64"
        if !data_uri.starts_with("data:application/octet-stream;base64") {
            panic!("Unsupported data URI encountered: {}", data_uri);
        }
    
        let data_in_base64 = data_uri.split_once(",").unwrap().1;
    
        base64::decode(data_in_base64).unwrap()
    }
}

/*
    Meshes in GLTF represents the data required for GPU draw calls.
*/
#[derive(Serialize, Deserialize, Debug)]
struct Mesh {
    name: String,
    primitives: Vec<Primitive>
}

/*
    Primitives are the actual structures that describes the data
    needed in order to make a GPU draw call for that primitive.
*/
#[derive(Serialize, Deserialize, Debug)]
struct Primitive {
    /*
        Each attribute is a value to an index of an accessor which
        contains the data for the attribute.
    */
    attributes: Attribute,
    /*
        Primitives that are indexed defines this indices property.
        This value is a reference to an accessor containing the corresponding data.
    */
    indices: u32
}

#[derive(Serialize, Deserialize, Debug)]
struct Attribute {
    #[serde(rename = "POSITION")]
    position: u32,
    #[serde(rename = "COLOR_0", default)]
    color_0: u32,
    #[serde(rename = "NORMAL", default)]
    normal: u32
}

#[derive(Debug)]
pub struct LoadedMesh {
    pub loaded_primitives: Vec<LoadedPrimitive>
}

#[derive(Debug)]
pub struct LoadedPrimitive {
    pub vertex_indices: LoadedBuffer,
    pub vertex_positions: LoadedBuffer,
    pub vertex_colors: Option<LoadedBuffer>,
    pub vertex_normals: Option<LoadedBuffer>
}

#[derive(Debug)]
pub struct LoadedBuffer {
    pub element_count: u32,
    pub buffer_data: Vec<u8>,
    pub buffer_component: BufferComponent,
    pub buffer_component_type: BufferComponentType
}

#[derive(Debug)]
pub enum BufferComponent {
    Vec3,
    Vec4,
    Scalar
}

#[derive(Debug)]
pub enum BufferComponentType {
    Float32,
    UnsignedShort,
    UnsignedShortNormalized
}