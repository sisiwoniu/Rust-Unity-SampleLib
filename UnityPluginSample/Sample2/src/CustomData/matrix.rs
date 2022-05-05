use super::vec3::Vec3;
use quaternion::axis_angle;
use std::{
    f32::consts::PI,
    ops::{Add, Deref, DerefMut, Mul},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RotateMatrix {
    pub x: [f32; 4],
    pub y: [f32; 4],
    pub z: [f32; 4],
    pub w: [f32; 4],
}

#[allow(dead_code)]
impl RotateMatrix {
    pub const ZERO: RotateMatrix = RotateMatrix {
        x: [1.0, 0.0, 0.0, 0.0],
        y: [0.0, 1.0, 0.0, 0.0],
        z: [0.0, 0.0, 1.0, 0.0],
        w: [0.0, 0.0, 0.0, 1.0],
    };

    pub fn get_x_axis_matrix(angle: f32) -> RotateMatrix {
        let x = [1.0, 0.0, 0.0, 0.0];
        let y = [0.0, angle.cos(), -angle.sin(), 0.0];
        let z = [0.0, angle.sin(), angle.cos(), 0.0];
        let w = [0.0, 0.0, 0.0, 1.0];

        RotateMatrix { x, y, z, w }
    }

    pub fn get_y_axis_matrix(angle: f32) -> RotateMatrix {
        let x = [angle.cos(), 0.0, angle.sin(), 0.0];
        let y = [0.0, 1.0, 0.0, 0.0];
        let z = [-angle.sin(), 0.0, angle.cos(), 0.0];
        let w = [0.0, 0.0, 0.0, 1.0];

        RotateMatrix { x, y, z, w }
    }

    pub fn get_z_axis_matrix(angle: f32) -> RotateMatrix {
        let x = [angle.cos(), -angle.sin(), 0.0, 0.0];
        let y = [angle.sin(), angle.cos(), 0.0, 0.0];
        let z = [0.0, 0.0, 1.0, 0.0];
        let w = [0.0, 0.0, 0.0, 1.0];

        RotateMatrix { x, y, z, w }
    }

    // pub fn to_quaternion(self, axis: RotateAxis) -> Quaternion {
    //     let vec3 = match axis {
    //         RotateAxis::AxisX => Some(Vec3 {
    //             x: 1.0,
    //             y: 0.0,
    //             z: 0.0,
    //         }),
    //         RotateAxis::AxisY => Some(Vec3 {
    //             x: 0.0,
    //             y: 1.0,
    //             z: 0.0,
    //         }),
    //         RotateAxis::AxisZ => Some(Vec3 {
    //             x: 0.0,
    //             y: 0.0,
    //             z: 1.0,
    //         }),
    //     };

    //     self * vec3::Vec3::ONE * vec3.unwrap()
    // }
}

impl Mul<Vec3> for RotateMatrix {
    type Output = Quaternion;

    fn mul(self, rhs: Vec3) -> Quaternion {
        let x = self.x[0] * rhs.x + self.x[1] * rhs.y + self.x[2] * rhs.z;
        let y = self.y[0] * rhs.x + self.y[1] * rhs.y + self.y[2] * rhs.z;
        let z = self.z[0] * rhs.x + self.z[1] * rhs.y + self.z[2] * rhs.z;
        let w = 1.0;

        Quaternion { x, y, z, w }
    }
}

impl Mul for RotateMatrix {
    type Output = Self;

    fn mul(self, rhs: RotateMatrix) -> Self {
        let x = [
            self.x[0] * rhs.x[0]
                + self.x[1] * rhs.y[0]
                + self.x[2] * rhs.z[0]
                + self.x[0] * rhs.w[0],
            self.x[0] * rhs.x[1]
                + self.x[1] * rhs.y[1]
                + self.x[2] * rhs.z[1]
                + self.x[3] * rhs.w[1],
            self.x[0] * rhs.x[2]
                + self.x[1] * rhs.y[2]
                + self.x[2] * rhs.z[2]
                + self.x[3] * rhs.w[2],
            self.x[0] * rhs.x[3]
                + self.x[1] * rhs.y[3]
                + self.x[2] * rhs.z[3]
                + self.x[3] * rhs.w[3],
        ];

        let y = [
            self.y[0] * rhs.x[0]
                + self.y[1] * rhs.y[0]
                + self.y[2] * rhs.z[0]
                + self.y[3] * rhs.w[0],
            self.y[0] * rhs.x[1]
                + self.y[1] * rhs.y[1]
                + self.y[2] * rhs.z[1]
                + self.y[3] * rhs.w[1],
            self.y[0] * rhs.x[2]
                + self.y[1] * rhs.y[2]
                + self.y[2] * rhs.z[2]
                + self.y[3] * rhs.w[2],
            self.y[0] * rhs.x[3]
                + self.y[1] * rhs.y[3]
                + self.y[2] * rhs.z[3]
                + self.y[3] * rhs.w[3],
        ];

        let z = [
            self.z[0] * rhs.x[0]
                + self.z[1] * rhs.y[0]
                + self.z[2] * rhs.z[0]
                + self.z[3] * rhs.w[0],
            self.z[0] * rhs.x[1]
                + self.z[1] * rhs.y[1]
                + self.z[2] * rhs.z[1]
                + self.z[3] * rhs.w[1],
            self.z[0] * rhs.x[2]
                + self.z[1] * rhs.y[2]
                + self.z[2] * rhs.z[2]
                + self.z[3] * rhs.w[2],
            self.z[0] * rhs.x[3]
                + self.z[1] * rhs.y[3]
                + self.z[2] * rhs.z[3]
                + self.z[3] * rhs.w[3],
        ];

        let w = [
            self.w[0] * rhs.x[0]
                + self.w[1] * rhs.y[0]
                + self.w[2] * rhs.z[0]
                + self.w[3] * rhs.w[0],
            self.w[0] * rhs.x[1]
                + self.w[1] * rhs.y[1]
                + self.w[2] * rhs.z[1]
                + self.w[3] * rhs.w[1],
            self.w[0] * rhs.x[2]
                + self.w[1] * rhs.y[2]
                + self.w[2] * rhs.z[2]
                + self.w[3] * rhs.w[2],
            self.w[0] * rhs.x[3]
                + self.w[1] * rhs.y[3]
                + self.w[2] * rhs.z[3]
                + self.w[3] * rhs.w[3],
        ];

        Self { x, y, z, w }
    }
}

#[allow(dead_code)]
pub enum RotateAxis {
    AxisX,
    AxisY,
    AxisZ,
}

//unity側に直接に渡すための定義
#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Add for Quaternion {
    type Output = Self;

    fn add(self, rhs: Quaternion) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Mul<Vec3> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Vec3) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w,
        }
    }
}

impl Deref for Quaternion {
    type Target = Self;

    fn deref(&self) -> &Self {
        &self
    }
}

impl DerefMut for Quaternion {
    fn deref_mut(&mut self) -> &mut Self {
        self
    }
}

#[allow(dead_code)]
impl Quaternion {
    pub fn rotate(angle: f32, axis: RotateAxis) -> Quaternion {
        let vec3 = match axis {
            RotateAxis::AxisX => Some(Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }),
            RotateAxis::AxisY => Some(Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }),
            RotateAxis::AxisZ => Some(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            }),
        };

        let rad = angle * (PI / 180.0);

        let result = axis_angle(vec3.unwrap().to_vector3(), rad);

        Quaternion {
            x: result.1[0],
            y: result.1[1],
            z: result.1[2],
            w: result.0,
        }
    }

    pub fn rotateX(angle: f32) -> Quaternion {
        Quaternion::rotate(angle, RotateAxis::AxisX)
    }

    pub fn rotateY(angle: f32) -> Quaternion {
        Quaternion::rotate(angle, RotateAxis::AxisY)
    }

    pub fn rotateZ(angle: f32) -> Quaternion {
        Quaternion::rotate(angle, RotateAxis::AxisZ)
    }

    pub fn copy(&mut self, dst: Quaternion) {
        self.x = dst.x;
        self.y = dst.y;
        self.z = dst.z;
        self.w = dst.w;
    }
}
