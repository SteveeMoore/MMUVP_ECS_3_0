#![allow(dead_code)]

use nalgebra::{Matrix3, Matrix6};

#[derive(Default)]
pub struct TensorL{
    pub tensor:Matrix3<f64>,
}

#[derive(Default)]
pub struct TensorD{
    pub tensor:Matrix3<f64>,
}

#[derive(Default)]
pub struct TensorDe{
    pub tensor:Matrix3<f64>,
}

#[derive(Default)]
pub struct TensorDin{
    pub tensor:Matrix3<f64>,
}

#[derive(Default)]
pub struct TensorEps{
    pub tensor:Matrix3<f64>,
}

#[derive(Default)]
pub struct TensorP{
    pub tensor: Matrix6<f64>,
}

#[derive(Default)]
pub struct TensorSigmaRate{
    pub tensor: Matrix3<f64>,
}

#[derive(Default)]
pub struct TensorSigma{
    pub tensor: Matrix3<f64>,
}

#[derive(Default)]
pub struct IntMeanSigma{
    pub value:f64,
}

#[derive(Default)]
pub struct IntMeanEps{
    pub value:f64,
}