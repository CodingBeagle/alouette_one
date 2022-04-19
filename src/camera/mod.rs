use crate::beagle_math;

#[derive(Default)]
pub struct Fps {
    delta_pitch: f32,
    delta_yaw: f32,
    delta_translation: beagle_math::Vector3,
    current_view_matrix: beagle_math::Mat4
}

impl Fps {
    pub fn apply_move(&mut self, delta_pitch: f32, delta_yaw: f32, delta_translate: beagle_math::Vector3) {
        self.delta_pitch += delta_pitch;
        self.delta_yaw += delta_yaw;
        self.delta_translation = self.delta_translation.add(&delta_translate);
    }

    pub fn view_matrix(&mut self) -> beagle_math::Mat4 {
        let pitch_axis = beagle_math::Vector3::new(1.0, 0.0, 0.0);
        let yaw_axis = beagle_math::Vector3::new(0.0, 1.0, 0.0);

        let mut pitch = beagle_math::Quaternion::default();
        let mut yaw = beagle_math::Quaternion::default();

        pitch.set_rotation(pitch_axis, self.delta_pitch);
        yaw.set_rotation(yaw_axis, self.delta_yaw);

        let rotation = pitch.cross(&yaw).to_matrix().get_transposed();
        let translation_matrix = beagle_math::Mat4::translate(&self.delta_translation.mul(-1.0));

        self.current_view_matrix = translation_matrix.mul(&rotation);

        beagle_math::Mat4::new(self.current_view_matrix.matrix)
    }

    pub fn forward(&self) -> beagle_math::Vector3 {
        let transposed = self.current_view_matrix.get_transposed();
        beagle_math::Vector3::new(
            transposed.get(0, 2),
            transposed.get(1, 2),
            transposed.get(2, 2))
    }

    pub fn right(&self) -> beagle_math::Vector3 {
        let transposed = self.current_view_matrix.get_transposed();
        beagle_math::Vector3::new(
            transposed.get(0, 0),
            transposed.get(1, 0),
            transposed.get(2, 0))
    }

    pub fn translation(&self) -> beagle_math::Vector3 {
        self.delta_translation
    }
}

#[derive(Default)]
pub struct FreeFlight {
    delta_pitch: f32,
    delta_yaw: f32,
    delta_roll: f32,
    delta_translate: beagle_math::Vector3,
    current_view_matrix: beagle_math::Mat4,
    current_trans: beagle_math::Mat4
}

impl FreeFlight {
    pub fn reset_orientation(&mut self) {
        self.current_view_matrix = beagle_math::Mat4::identity();
    }

    pub fn apply_move(&mut self, delta_pitch: f32, delta_yaw: f32, delta_roll: f32, delta_translation: beagle_math::Vector3) {
            self.delta_pitch = delta_pitch;
            self.delta_yaw = delta_yaw;
            self.delta_roll = delta_roll;
            self.delta_translate = delta_translation;
    }

    pub fn get_position(&self) -> beagle_math::Vector3 {
        let x = self.current_view_matrix.get(0, 3);
        let y = self.current_view_matrix.get(1, 3);
        let z = self.current_view_matrix.get(2, 3);

        let v = beagle_math::Vector4::new(x, y, z, 1.0);

        /*
            Here I take the translation part of the current view matrix, which expresses the displacement, in world coordinates, that all objects in the scene
            should displace by RELATIVE TO THE CAMERA AS ORIGIN, in order to simulate the current orientation of the FreeFlight camera. 

            I then take that world coordinate displacement, and convert it into a displacement relative to the coordinate space of the current view, that is,
            with the current view matrix as the origin.

            The resulting position is essentially the absolute world position of the FreeFlight camera.

            I need an absolute position like this when calculating things like specular light in the vertex shader, as this requires a position of the eye in world coordinates.
            And the base self.current_view_matrix stores no such information.
        */
        let loc = beagle_math::Mat4::parent_to_local(&beagle_math::Vector3::new(v.x, v.y, v.z), &self.current_view_matrix);

        let res = beagle_math::Vector3::new(loc.get(0, 3), loc.get(1, 3), loc.get(2, 3));

        res
    }

    pub fn view_matrix(&mut self) -> beagle_math::Mat4 {
        let yaw_axis = beagle_math::Vector3::new(0.0, 1.0, 0.0);
        let pitch_axis = beagle_math::Vector3::new(1.0, 0.0, 0.0);
        let roll_axis = beagle_math::Vector3::new(0.0, 0.0, 1.0);

        let mut yaw = beagle_math::Quaternion::default();
        let mut pitch = beagle_math::Quaternion::default();
        let mut roll = beagle_math::Quaternion::default();

        yaw.set_rotation(yaw_axis, self.delta_yaw);
        pitch.set_rotation(pitch_axis, self.delta_pitch);
        roll.set_rotation(roll_axis, self.delta_roll);

        let rotation = yaw.cross(&pitch).cross(&roll).to_matrix().get_transposed();
        let translation_matrix = beagle_math::Mat4::translate(&self.delta_translate.mul(-1.0));

        self.current_trans = self.current_trans.mul(&translation_matrix);

        self.current_view_matrix = self.current_view_matrix.mul(&(translation_matrix.mul(&rotation)));
        
        beagle_math::Mat4::new(self.current_view_matrix.matrix)
    }
}