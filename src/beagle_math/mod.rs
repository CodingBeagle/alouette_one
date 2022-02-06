use std::fmt::{self};

// The "marker" module contains primitive traits and types representing basic properties of types.
// Need the trait "Copy"
use std::marker::{Copy};

/*
    NOTICE:

    My Math API currently has transform and projection matrices design in terms of:

    - Row-Major Vector Convention: Not talking about memory layout here. Meaning, my matrices are designed from the point of view of being multiplied by ROW vectors.
    - Projection and View matrices currently assume Left-Handedness, as this is what is ultimately assumed by the Rasterizer stage of DirectX 11.
*/

#[derive(Default, Clone, Copy)]
pub struct Vector2
{
    pub x: f32,
    pub y: f32
}

impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vector2")
        .field("x", &self.x)
        .field("y", &self.y)
        .finish()
    }
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 {
            x,
            y
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Vector3
{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vector3")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {
            x,
            y,
            z
        }
    }

    pub fn add(&self, vec: &Vector3) -> Vector3 {
        Vector3::new(
            self.x + vec.x,
            self.y + vec.y,
            self.z + vec.z)
    }

    pub fn cross(&self, vec: &Vector3) -> Vector3 {
        Vector3::new(
            self.y * vec.z - self.z * vec.y,
            self.z * vec.x - self.x * vec.z,
            self.x * vec.y - self.y * vec.x)
    }

    pub fn dot(&self, vec: &Vector3) -> f32 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    pub fn mul(&self, scalar: f32) -> Vector3 {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

#[derive(Default, Clone, Copy)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl fmt::Debug for Vector4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vector4")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("w", &self.w)
            .finish()
    }
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 {
            x,
            y,
            z,
            w
        }
    }

    pub fn dot(&self, vec: &Vector4) -> f32 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z + self.w * vec.w
    }

    pub fn magnitude(&self) -> f32 {
        ( self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0) + self.w.powf(2.0) ).sqrt()
    }

    pub fn normalize(&self) -> Vector4 {
        Vector4::new( self.x / self.magnitude(), self.y / self.magnitude(), self.z / self.magnitude(), self.w / self.magnitude() )
    }

    pub fn as_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

pub struct Mat4
{
    matrix: [f32; 16]
}

impl fmt::Debug for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}\n{:?}\n{:?}\n{:?}\n", 
            Vector4::new(self.get(0, 0), self.get(1, 0), self.get(2, 0), self.get(3, 0)),
            Vector4::new(self.get(0, 1), self.get(1, 1), self.get(2, 1), self.get(3, 1)),
            Vector4::new(self.get(0, 2), self.get(1, 2), self.get(2, 2), self.get(3, 2)),
            Vector4::new(self.get(0, 3), self.get(1, 3), self.get(2, 3), self.get(3, 3)))
    }
}

// Methods in Rust are defined within an "implementation block" for a specified struct.
impl Mat4
{
    // An "associated function" is like a static method, in that it is not associated with a specific instance of a struct,
    // but associated with the type itself.
    pub fn new(matrix: [f32; 16]) -> Mat4 {
        Mat4 {
            matrix
        }
    }

    pub fn get(&self, x: i32, y: i32) -> f32 {
        let index : usize = (x + (4 * y)) as usize;
        self.matrix[index]
    }

    pub fn identity() -> Mat4 {
        Mat4 {
            matrix: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    pub fn translate(pos: &Vector3) -> Mat4 {
        Mat4 {
            matrix: [
                1.0  , 0.0  , 0.0  , 0.0,
                0.0  , 1.0  , 0.0  , 0.0,
                0.0  , 0.0  , 1.0  , 0.0,
                pos.x  , pos.y  , pos.z , 1.0
            ]
        }
    }

    pub fn scale(scale: &Vector3) -> Mat4 {
        Mat4 {
            matrix: [
                scale.x, 0.0    , 0.0    , 0.0,
                0.0    , scale.y, 0.0    , 0.0,
                0.0    , 0.0    , scale.z, 0.0,
                0.0    , 0.0    , 0.0    , 1.0
            ]
        }
    }

    pub fn rotate_x(rad: f32) -> Mat4 {
        Mat4 {
            matrix: [
                1.0, 0.0, 0.0, 0.0,
                0.0, rad.cos(), rad.sin(), 0.0,
                0.0, -rad.sin(), rad.cos(), 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    pub fn rotate_y(rad: f32) -> Mat4 {
        Mat4 {
            matrix: [
                rad.cos() , 0.0 , -rad.sin() , 0.0,
                0.0       , 1.0 , 0.0       , 0.0,
                rad.sin(), 0.0 , rad.cos() , 0.0,
                0.0       , 0.0 , 0.0       , 1.0
            ]
        }
    }

    pub fn rotate_z(rad: f32) -> Mat4 {
        Mat4 {
            matrix: [
                rad.cos(), rad.sin(), 0.0, 0.0,
                -rad.sin(), rad.cos(), 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    pub fn mul_row(&self, row: &Vector4) -> Vector4 {
        Vector4::new(
            row.x * self.matrix[0] + row.y * self.matrix[4] + row.z * self.matrix[8] + row.w * self.matrix[12],
            row.x * self.matrix[1] + row.y * self.matrix[5] + row.z * self.matrix[9] + row.w * self.matrix[13],
            row.x * self.matrix[2] + row.y * self.matrix[6] + row.z * self.matrix[10] + row.w * self.matrix[14],
            row.x * self.matrix[3] + row.y * self.matrix[7] + row.z * self.matrix[11] + row.w * self.matrix[15])
    }

    pub fn tranpose(&mut self) {
        self.matrix = self.get_column_major_value()
    }

    // For this projection matrix, I use what is sometimes referred to as the Hor+ scaling method for Field of View (https://en.wikipedia.org/wiki/Field_of_view_in_video_games).
    // Basically, the vertical FoV is fixed, while the horizontal FoV scales with the aspect ratio.
    pub fn projection(fov: f32, width: f32, height: f32, near: f32, far: f32) -> Mat4 {
        let y_scale = 1.0 / (fov * 0.5).tan();
        let x_scale = y_scale / (width / height);
        let q = far / (far - near);

        /*
            This is a left-handed perspective projection.
        */
        Mat4 {
            matrix: [
                x_scale, 0.0    , 0.0      , 0.0,
                0.0    , y_scale, 0.0      , 0.0,
                0.0    , 0.0    , q        , 1.0,
                0.0    , 0.0    , -q * near, 0.0
            ]
        }
    }

    pub fn mul(&self, mat: &Mat4) -> Mat4 {
        let self_row0 = Vector4::new(self.get(0, 0), self.get(1, 0),  self.get(2, 0), self.get(3, 0));
        let self_row1 = Vector4::new(self.get(0, 1), self.get(1, 1),  self.get(2, 1), self.get(3, 1));
        let self_row2 = Vector4::new(self.get(0, 2), self.get(1, 2),  self.get(2, 2), self.get(3, 2));
        let self_row3 = Vector4::new(self.get(0, 3), self.get(1, 3),  self.get(2, 3), self.get(3, 3));

        let mat_column0 = Vector4::new( mat.get(0, 0), mat.get(0, 1), mat.get(0, 2), mat.get(0, 3));
        let mat_column1 = Vector4::new( mat.get(1, 0), mat.get(1, 1), mat.get(1, 2), mat.get(1, 3));
        let mat_column2 = Vector4::new( mat.get(2, 0), mat.get(2, 1), mat.get(2, 2), mat.get(2, 3));
        let mat_column3 = Vector4::new( mat.get(3, 0), mat.get(3, 1), mat.get(3, 2), mat.get(3, 3));

        Mat4 {
            matrix: [
                self_row0.dot(&mat_column0), self_row0.dot(&mat_column1), self_row0.dot(&mat_column2), self_row0.dot(&&mat_column3),
                self_row1.dot(&mat_column0), self_row1.dot(&mat_column1), self_row1.dot(&mat_column2), self_row1.dot(&&mat_column3),
                self_row2.dot(&mat_column0), self_row2.dot(&mat_column1), self_row2.dot(&mat_column2), self_row2.dot(&&mat_column3),
                self_row3.dot(&mat_column0), self_row3.dot(&mat_column1), self_row3.dot(&mat_column2), self_row3.dot(&&mat_column3),
            ]
        }
    }

    pub fn get_value(&self) -> [f32; 16] {
        self.matrix
    }

    pub fn get_column_major_value(&self) -> [f32; 16] {
        [
            self.matrix[0], self.matrix[4], self.matrix[8], self.matrix[12],
            self.matrix[1], self.matrix[5], self.matrix[9], self.matrix[13],
            self.matrix[2], self.matrix[6], self.matrix[10], self.matrix[14],
            self.matrix[3], self.matrix[7], self.matrix[11], self.matrix[15]
        ]
    }
}

#[derive(Default)]
pub struct Quaternion {
    w: f32,
    v: Vector3
}

impl Quaternion {
    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Quaternion {
        Quaternion {
            w,
            v: Vector3::new(x, y, z)
        }
    }

    pub fn set_rotation(&mut self, axis: Vector3, angle_in_radians: f32)
    {
        let angle_in_radians = angle_in_radians / 2f32;

        self.w = angle_in_radians.cos();
        self.v = Vector3::new(
            angle_in_radians.sin() * axis.x,
            angle_in_radians.sin() * axis.y,
            angle_in_radians.sin() * axis.z);
    }

    // TODO:
    //  Currently using standard formula for quaternion cross product.
    //  This means that you have to remember to apply quaternions "backwards",
    //  From "right to left".
    //  That is, if you want to multiply two quaternions q1 * q2,
    //  This function call would have to look like: q2.cross(&q1)
    pub fn cross(&self, other: &Quaternion) -> Quaternion {
        let v1 = self.v;
        let v2 = other.v;

        let mut v3 = v2.mul(self.w);
        v3 = v3.add(&v1.mul(other.w));
        v3 = v3.add(&v2.cross(&v1));

        Quaternion::new(
            self.w * other.w - v1.dot(&v2),
            v3.x,
            v3.y,
            v3.z)
    }

    pub fn to_matrix(&self) -> Mat4 {
        let q = Vector4::new(self.v.x, self.v.y, self.v.z, self.w).normalize();

        let m11 = 1.0 - 2.0 * q.y.powf(2.0) - 2.0 * q.z.powf(2.0);
        let m12 = 2.0 * q.x * q.y + 2.0 * q.z * q.w;
        let m13 = 2.0 * q.x * q.z - 2.0 * q.y * q.w;
        let m14 = 0.0;

        let m21 = 2.0 * q.x * q.y - 2.0 * q.z * q.w;
        let m22 = 1.0 - 2.0 * q.x.powf(2.0) - 2.0 * q.z.powf(2.0);
        let m23 = 2.0 * q.y * q.z + 2.0 * q.x * q.w;
        let m24 = 0.0;

        let m31 = 2.0 * q.x * q.z + 2.0 * q.y * q.w;
        let m32 = 2.0 * q.y * q.z - 2.0 * q.x * q.w;
        let m33 = 1.0 - 2.0 * q.x.powf(2.0) - 2.0 * q.y.powf(2.0);
        let m34 = 0.0;

        let m41 = 0.0;
        let m42 = 0.0;
        let m43 = 0.0;
        let m44 = 1.0;

        let mut m = Mat4::new([
            m11, m12, m13, m14,
            m21, m22, m23, m24,
            m31, m32, m33, m34,
            m41, m42, m43, m44
        ]);

        m
    }

    pub fn Rotation(axis: Vector3, angle_in_radians: f32) -> Mat4 {
        let angle = angle_in_radians / 2.0;

        let mut q = Vector4::new(
            angle.sin() * axis.x,
            angle.sin() * axis.y,
            angle.sin() * axis.z,
            angle.cos()
        );

        q = q.normalize();

        let m11 = 1.0 - 2.0 * q.y.powf(2.0) - 2.0 * q.z.powf(2.0);
        let m12 = 2.0 * q.x * q.y + 2.0 * q.z * q.w;
        let m13 = 2.0 * q.x * q.z - 2.0 * q.y * q.w;
        let m14 = 0.0;

        let m21 = 2.0 * q.x * q.y - 2.0 * q.z * q.w;
        let m22 = 1.0 - 2.0 * q.x.powf(2.0) - 2.0 * q.z.powf(2.0);
        let m23 = 2.0 * q.y * q.z + 2.0 * q.x * q.w;
        let m24 = 0.0;

        let m31 = 2.0 * q.x * q.z + 2.0 * q.y * q.w;
        let m32 = 2.0 * q.y * q.z - 2.0 * q.x * q.w;
        let m33 = 1.0 - 2.0 * q.x.powf(2.0) - 2.0 * q.y.powf(2.0);
        let m34 = 0.0;

        let m41 = 0.0;
        let m42 = 0.0;
        let m43 = 0.0;
        let m44 = 1.0;

        let m = Mat4::new([
            m11, m12, m13, m14,
            m21, m22, m23, m24,
            m31, m32, m33, m34,
            m41, m42, m43, m44
        ]);

        m
    }
}

#[cfg(test)]
mod tests {
    use crate::beagle_math::*;

    #[test]
    fn tmp_quick_tester() {
        let mat_a = Mat4::new([
            1.0, 8.0, 2.0, 3.0,
            2.0, 1.0, 1.0, 1.0,
            3.0, 2.0, 4.0, 3.0,
            1.0, 9.0, 4.0, 3.0
        ]);

        let mat_b = Mat4::new([
            8.0, 4.0, 3.0, 2.0,
            1.0, 1.0, 2.0, 2.0,
            3.0, 2.0, 1.0, 1.0,
            1.0, 2.0, 3.0, 4.0
        ]);

        let result = mat_a.mul(&&mat_b);

        let mut rofl = Mat4::new([
            5.0, 6.0, 1.0, 2.0,
            3.0, 0.0, 12.0, 2.0,
            48.0, 38.0, 2.0, 9.0,
            8.0, 5.0, 4.0, 44.0
        ]);

        rofl.tranpose();

        println!("{:?}", rofl);
    }

    #[test]
    fn should_print_matrix_debug_output_when_using_debug_formatter() {
        let my_matrix = Mat4::identity();

        print!("{:?}", my_matrix);

        assert!(true);
    }

    #[test]
    fn should_return_correct_identity_matrix_when_constructing_identity_matrix() {
        // Arrange
        let expected_matrix: [f32; 16] = [1.0, 0.0, 0.0, 0.0,
                                            0.0, 1.0, 0.0, 0.0,
                                            0.0, 0.0, 1.0, 0.0,
                                            0.0, 0.0, 0.0, 1.0];

        // Act
        let value =  Mat4::identity().get_value();

        // Assert
        assert!( value.iter().eq(expected_matrix.iter()), "The identity matrix was found to be incorrect!");
    }

    #[test]
    fn should_return_correct_column_major_matrix_when_getting_column_major_value() {
        // Arrange
        let original_matrix = Mat4::new(
                                    [11.0, 12.0, 13.0, 14.0,
                                            21.0, 22.0, 23.0, 24.0,
                                            31.0, 32.0, 33.0, 34.0,
                                            41.0, 42.0, 43.0, 44.0]);

        let expected_matrix: [f32; 16] = [11.0, 21.0, 31.0, 41.0,
                                            12.0, 22.0, 32.0, 42.0,
                                            13.0, 23.0, 33.0, 43.0,
                                            14.0, 24.0, 34.0, 44.0];

        // Act
        let matrix_in_column_major = original_matrix.get_column_major_value();

        // Assert
        assert!( matrix_in_column_major.iter().eq(expected_matrix.iter()) );
    }

    #[test]
    fn should_calculate_magnitude_correctly() {
        let my_vector = Vector4::new(2.0, 4.0, 7.0, 8.0);

        let magnitude = my_vector.magnitude();

        println!("magnitude: {}", magnitude);

        let normalized = my_vector.normalize();

        println!("normalized: {:?}", normalized);
    }

    #[test]
    fn should_calculate_crossproduct_correctly() {
        let vec_a = Vector3::new(2.0, 3.0, 4.0);
        let vec_b = Vector3::new(5.0, 6.0, 7.0);

        let cross_product = vec_a.cross(&vec_b);

        assert!(cross_product.x.eq(&-3f32));
        assert!(cross_product.y.eq(&6f32));
        assert!(cross_product.z.eq(&-3f32));
    }
    
    #[test]
    fn should_calculate_dotproduct_correctly()
    {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(1.0, 5.0, 7.0);

        let dotproduct = v1.dot(&v2);

        assert!(dotproduct.eq(&32.0f32));
    }

    #[test]
    fn should_calculate_row_mul_correctly()
    {
        let v = Vector4::new(1.0, 2.0, 3.0, 4.0);
        
        let m = Mat4::new([
            2.0, 6.0, 7.0, 8.0,
            1.0, 2.0, 0.0, 1.0,
            2.0, 1.0, 4.0, 4.0,
            6.0, 6.0, 7.0, 7.0
        ]);

        let result = m.mul_row(&v);

        assert!(result.x.eq(&34.0f32));
        assert!(result.y.eq(&37.0f32));
        assert!(result.z.eq(&47.0f32));
        assert!(result.w.eq(&50.0f32));
    }

    #[test]
    fn should_calculate_quat_crossproduct_correctly() {
        let q1 = Quaternion::new(1.0, 2.0, 1.0, 4.0);
        let q2 = Quaternion::new(2.0, 6.0, 3.0, 2.0);

        let cross = q2.cross(&q1);

        println!("Quat is:");
        println!("{}, {}, {}, {}", cross.w, cross.v.x, cross.v.y, cross.v.z);
    }
}