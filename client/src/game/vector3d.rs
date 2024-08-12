use std::ops::{Add, Sub, Mul, Neg};

use bevy::math::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Vector3D) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vector3D {
        let mag = self.magnitude();
        Vector3D {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }
    pub fn from_v3(v : Vec3) -> Self {
       Self { x: v.x, y: v.y, z: v.z }
    }
    pub fn from_v3b(v : crate::Vec3) -> Self {
        Self { x: v.x, y: v.y, z: v.z }
    }
    pub fn to_v3(a : Self) -> Vec3 {
        Vec3::new( a.x, a.y, a.z)
    }
    // bevy::prelude::Vec3
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vector3D {
    type Output = Vector3D;

    fn mul(self, scalar: f32) -> Vector3D {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Vector3D {
        Vector3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
