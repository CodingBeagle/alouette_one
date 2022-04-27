use std::fs;
use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct File {
    #[serde(default)]
    pub nodes: Vec<Node>,

    #[serde(default)]
    pub materials: Vec<Material>,

    #[serde(default)]
    pub meshes: Vec<Mesh>,

    #[serde(default)]
    pub accessors: Vec<Accessor>,

    #[serde(default)]
    pub buffer_views: Vec<BufferView>,

    #[serde(default)]
    pub buffers: Vec<Buffer>
}

impl File {
    // TODO: Returning an error as a string isn't very clear... perhaps a strong type for it would make the interface more easily understandable?
    pub fn from(file_path: PathBuf) -> Result<File, String> {
        let file_content = match fs::read_to_string(file_path) {
            Ok(file_content) => file_content,
            Err(err) => return Err(format!("Failed to read GLTF File: {}", err.to_string()))
        };

        match serde_json::from_str::<File>(&file_content) {
            Ok(json) => Ok(json),
            Err(err) => Err(format!("Failed to deserialize GLTF file: {}", err.to_string()))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    #[serde(default)]
    pub children: Vec<i32>,

    #[serde(default)]
    pub mesh: i32,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub translation: [f32; 3],

    #[serde(default)]
    pub scale: [f32; 3],

    #[serde(default)]
    pub rotation: [f32; 4]
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub extras: Extra
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Mesh {
    #[serde(default)]
    pub name: String,

    pub primitives: Vec<Primitive>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Primitive {
    pub attributes: Attribute,
    
    #[serde(default)]
    pub indices: u32,

    #[serde(default = "default_attribute_value")]
    pub material: i32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    #[serde(default = "default_attribute_value", rename = "POSITION")]
    pub position: i32,

    #[serde(default = "default_attribute_value", rename = "COLOR_0")]
    pub color_0: i32,

    #[serde(default = "default_attribute_value", rename = "NORMAL")]
    pub normal: i32,

    #[serde(default = "default_attribute_value", rename = "TEXCOORD_0")]
    pub texcoord_0: i32
}

// TODO: Not entirely sure about the best way to represent an absense of integer values in file...
// Perhaps you can use an Option type with Serde? Then absense would be easily indicated.
fn default_attribute_value() -> i32 {
    -1
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Accessor {
        #[serde(default)]
        pub buffer_view: u32,

        // The data type of each individual value (component)
        // 5123 = Unsigned Short, 16 bits, 2 bytes
        // 5126 = float, 32 bits, 4 bytes
        pub component_type: u32,

        // Count is the number of elements in the buffer
        pub count: u32,

        // The normalized bool indicates whether the value has to be normalized before use.
        // That is, if the integer value has to be divided by its types MAX value before being used, in order to give a number between
        // [0, 1] for unsigned integer types, and [-1, 1] for signed integer types.
        #[serde(default)]
        pub normalized: bool,

        // Renamed because "type" is a Rust keyword
        #[serde(rename = "type")]
        pub element_type: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BufferView {
    // A reference index to an underlying buffer.
    pub buffer: u32,

    // The amount of bytes in the buffer that this view cares about
    pub byte_length: u32,

    // The start offset in bytes for this buffer view.
    #[serde(default)]
    pub byte_offset: u32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Buffer {
    pub byte_length: u32,

    #[serde(default)]
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Extra {
    #[serde(default)]
    pub diffuse: [f32; 3],

    #[serde(default)]
    pub specular: [f32; 3],

    #[serde(default)]
    pub ambient: [f32; 3],
    
    #[serde(default)]
    pub shininess_factor: f32
}