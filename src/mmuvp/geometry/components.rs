#![allow(dead_code)]

pub struct Volume{
    pub value:f64,
}
impl Default for Volume{
    fn default() -> Self {
        Self{value: 1.0}
    }
}

pub struct Radius{
    pub value:f64,
}
impl Default for Radius{
    fn default() -> Self {
        Self{value: 1.0}
    }
}