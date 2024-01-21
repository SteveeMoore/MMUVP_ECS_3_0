#![allow(dead_code)]

use crate::entities::Entity;

#[derive(Default)]
pub struct StatusRecrystComponent{
    pub status:bool,
}

#[derive(Default)]
pub struct AccumEnergy{
    pub value:f64,
}

#[derive(Default)]
pub struct AccumEnergyRate{
    pub value:f64,
}

#[derive(Default)]
pub struct Subgrains{
    pub vector:Vec<Entity>,
}

#[derive(Default)]
pub struct DriveForce{
    pub value:f64,
}

#[derive(Default)]
pub struct FacetMobility{
    pub value:f64,
}

#[derive(Default)]
pub struct VelocityFacet{
    pub value: f64,
}