#![allow(dead_code)]
use nalgebra::Matrix3;

pub struct Rotation{
    pub tensor: Matrix3<f64>,
}

impl Default for Rotation {
    fn default() -> Self {
        Self{tensor: Matrix3::identity()}
    }
}