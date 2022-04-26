use std::mem::size_of;
use byteorder::{LittleEndian, ByteOrder};

pub trait FromBinary: Sized {
    // TODO: Perhaps it should return a type of Result class with success or error!
    fn from_binary(binary: &[u8]) -> Self;
    fn from_binary_collection(binary: &[u8]) -> Vec<Self>;
}

impl FromBinary for u16 {
    fn from_binary(binary: &[u8]) -> Self {
        if binary.len() % size_of::<u16>() != 0 {
            panic!("Length of binary buffer is not divisible by byte length of u32, which is {}", size_of::<u32>());
        }

        LittleEndian::read_u16(binary)
    }

    fn from_binary_collection(binary: &[u8]) -> Vec<Self> {
        let size_of_u16_in_bytes = size_of::<u16>();
        if binary.len() % size_of_u16_in_bytes != 0 {
            panic!("Length of binary buffer is not divisible by byte length of u32, which is {}", size_of_u16_in_bytes);
        }

        let binary_data_length = binary.len();
        let mut result : Vec<u16> = vec!();
        for i in (0..binary_data_length).step_by(size_of_u16_in_bytes) {
            let slice_start_offset = i;
            let slice_end_offset = slice_start_offset + size_of_u16_in_bytes;

            result.push(LittleEndian::read_u16(&binary[slice_start_offset..slice_end_offset]));
        }

        result
    }
}

impl FromBinary for u32 {
    fn from_binary(binary: &[u8]) -> Self {
        if binary.len() % size_of::<u32>() != 0 {
            panic!("Length of binary buffer is not divisible by byte length of u32, which is {}", size_of::<u32>());
        }

        LittleEndian::read_u32(binary)
    }

    fn from_binary_collection(binary: &[u8]) -> Vec<Self> {
        let size_of_u32_in_bytes = size_of::<u32>();
        if binary.len() % size_of_u32_in_bytes != 0 {
            panic!("Length of binary buffer is not divisible by byte length of u32, which is {}", size_of_u32_in_bytes);
        }

        let binary_data_length = binary.len();
        let mut result : Vec<u32> = vec!();
        for i in (0..binary_data_length).step_by(size_of_u32_in_bytes) {
            let slice_start_offset = i;
            let slice_end_offset = slice_start_offset + size_of_u32_in_bytes;

            result.push(LittleEndian::read_u32(&binary[slice_start_offset..slice_end_offset]));
        }

        result
    }
}