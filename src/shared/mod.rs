pub trait FromBinary: Sized {
    // TODO: Perhaps it should return a type of Result class with success or error!
    fn from_binary(binary: &[u8]) -> Self;
    fn from_binary_collection(binary: &[u8]) -> Vec<Self>;
}