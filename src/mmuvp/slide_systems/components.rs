#![allow(dead_code)]

use nalgebra::{Vector3, Matrix3};

#[derive(Default)]
pub struct BurgersVectors{
    pub vectors:Vec<Vector3<f64>>,
}

#[derive(Default)]
pub struct NormalsVectors{
    pub vectors:Vec<Vector3<f64>>,
}

#[derive(Default)]
pub struct BNMatrix{
    pub tensors:Vec<Matrix3<f64>>,
}


pub struct Tau{
    pub values:Vec<f64>,
}
impl Default for Tau {
    fn default() -> Self {
        Self{values: vec![0.0; 24]}
    }
}

pub struct TauCRate{
    pub values:Vec<f64>,
}
impl Default for TauCRate {
    fn default() -> Self {
        Self{values: vec![0.0; 24]}
    }
}

pub struct TauC{
    pub values:Vec<f64>,
}
impl Default for TauC {
    fn default() -> Self {
        Self{values: vec![0.0; 24]}
    }
}

pub struct TauCHP{
    pub values:Vec<f64>,
}
impl Default for TauCHP {
    fn default() -> Self {
        Self{values: vec![0.0; 24]}
    }
}

pub struct Gamma{
    pub values:Vec<f64>,
}
impl Default for Gamma {
    fn default() -> Self {
        Self{values: vec![0.0; 24]}
    }
}

pub struct GammaRate{
    pub values:Vec<f64>,
}
impl Default for GammaRate {
    fn default() -> Self {
        Self{values: vec![0.0; 24]}
    }
}


pub struct HVector{
    pub vector:Vec<f64>,
}
impl Default for HVector {
    fn default() -> Self {
        Self{vector: vec![0.0; 24]}
    }
}

pub struct HMatrix{
    pub matrix:Vec<Vec<f64>>,
}
impl Default for HMatrix{
    fn default() -> Self {
        let mut matrix:Vec<Vec<f64>> = Vec::with_capacity(24);
        for _ in 0..24{
            let vector = vec![0.0; 24];
            matrix.push(vector);
        }
        Self{matrix}
    }
}