#![allow(dead_code)]

use std::f64::consts::PI;

use nalgebra::Matrix3;

use crate::{entities::Entity, entity, mmuvp::{geometry::{components::{Radius, Volume}, systems::GeometrySystem}, rotation::{components::Rotation, systems::RotationSystem}, params::components::Params}, TensorSigma, TensorDin, consts::MEGA, TensorEps, TensorDe, TensorD, TensorL, NormalsVectors, TensorP, TensorSigmaRate, BurgersVectors, BNMatrix, TauC, TauCRate, TauCHP, Tau, GammaRate, HVector, HMatrix, StrainSystem, StressSystem, SlideSystem, DislocationSlidingSystem};

use super::components::{Subgrains, DriveForce, AccumEnergy, AccumEnergyRate, FacetMobility, VelocityFacet};

pub struct SubgrainSystem;

impl SubgrainSystem {
    pub fn initialize(entities: &mut [Entity], num_sg: i64, r0:f64){
        for entity in entities.iter_mut(){
            if let Some(subgr) = entity.get_component_mut::<Subgrains>(){
                for i in 0..num_sg{
                    subgr.vector.push(entity!(i as usize, Radius, Volume, DriveForce));
                }
                GeometrySystem::initialize_rey(&mut subgr.vector, r0);
            }
        }
    }    
}

pub struct RecrystallizationSystem;

impl RecrystallizationSystem{
    pub fn calc_accum_energy_rate(entities:&mut [Entity], alfa:f64){
        for entity in entities.iter_mut(){
            let sigma = entity.get_component::<TensorSigma>().unwrap().tensor.clone();
            let din = entity.get_component::<TensorDin>().unwrap().tensor.clone();
            if let Some(est_rate) = entity.get_component_mut::<AccumEnergyRate>(){
                let sigma_pa= sigma*MEGA;
                est_rate.value = alfa* (sigma_pa.dot(&din));
            }
        }
    }

    pub fn calc_accum_energy(entities:&mut [Entity], dt:f64){
        for entity in entities.iter_mut(){
            let est_rate = entity.get_component::<AccumEnergyRate>().unwrap().value;
            if let Some(est) = entity.get_component_mut::<AccumEnergy>(){
                let est_old = est.value;
                let est_new = est_old + est_rate*dt;
                est.value = est_new;
            }
        }
    }

    pub fn calc_mean_accum_energy(output_entity: &mut Entity, entities: & [Entity], ){
        let mut summ = 0.0;
        let mut summvol = 0.0;
        for entity in entities.iter(){
            let volume = entity.get_component::<Volume>().unwrap().value;
            summvol+=volume;
            let est = entity.get_component::<AccumEnergy>().unwrap().value;
            summ += est*volume;
        }
        let est_mean = output_entity.get_component_mut::<AccumEnergy>().unwrap();
        est_mean.value = summ/summvol;
    }

    pub fn calc_drive_force(entities:&mut [Entity], polycrystal_entity: &Entity, egb:f64){
        let est_poly = polycrystal_entity.get_component::<AccumEnergy>().unwrap().value;
        for entity in entities.iter_mut(){
            let gr_size = entity.get_component::<Radius>().unwrap().value;
            if let Some(drive_force) = entity.get_component_mut::<DriveForce>(){
                let value = est_poly - 3.0 * egb / gr_size;
                drive_force.value = value;              
            }

            
            if let Some(subgr) = entity.get_component_mut::<Subgrains>(){
                let subgr_entities = &mut subgr.vector;
                Self::calc_drive_force(subgr_entities, polycrystal_entity, egb);
                
            }
        }
    }

    pub fn initialize_facet_mobility(entities:&mut [Entity], m:f64, q: f64, r: f64, temp:f64){
        let value = m* (-q/(r*temp)).exp();
        for entity in entities.iter_mut(){
            if let Some(facet_mobility) = entity.get_component_mut::<FacetMobility>(){
                facet_mobility.value = value;
            }
        }
    }

    pub fn calc_velocity_facet(entities:&mut [Entity]){
        for entity in entities.iter_mut(){
            let facet_mobility = entity.get_component::<FacetMobility>().unwrap().value;
            let driveforce = entity.get_component::<DriveForce>().unwrap().value;
            if let Some(facet_velocity) = entity.get_component_mut::<VelocityFacet>(){
                if (facet_mobility * driveforce)> 0.0 {
                    facet_velocity.value = facet_mobility * driveforce;
                } else {
                    facet_velocity.value = 0.0;
                }   
                
            }
        }
    }


    pub fn calc_radius(entities:&mut [Entity], dt:f64){
        let mut dv = 0.0;
        for entity in entities.iter_mut(){
            if let Some(facet_velocity) = entity.get_component::<VelocityFacet>(){
                let vel_fac= facet_velocity.value;
                if let Some(radius) = entity.get_component_mut::<Radius>(){
                    if vel_fac>0.0{
                        radius.value += vel_fac * dt;
                    }
                }
                let r = entity.get_component::<Radius>().unwrap().value;
                if let Some(volume) = entity.get_component_mut::<Volume>(){
                    let value = 4.0/3.0 * PI * r.powf(3.0);
                    dv = volume.value - value;
                    volume.value = value;
                }            
            }
        }

        let mut victim_entity: Option<&mut Entity>=None;
        let mut max_est = 0.0;
        for entity in entities.iter_mut(){
            if entity.get_component::<AccumEnergy>().unwrap().value > max_est{
                max_est = entity.get_component::<AccumEnergy>().unwrap().value;
                victim_entity = Some(entity);
            }
        }

        if let Some(victim) = victim_entity{
            let mut new_value = 0.0;
            if let Some(vict_volume) = victim.get_component_mut::<Volume>(){
                new_value = vict_volume.value - dv;
                if new_value>=0.0{
                    vict_volume.value = new_value;
                } else {
                    vict_volume.value = 0.0;
                }
            }
            if let Some(vict_radius) = victim.get_component_mut::<Radius>(){
                vict_radius.value =  (3.0 / 4.0 / PI * new_value).powf(1.0/3.0);    
            }        
        }
               
    }

    pub fn remove_grain(entities:&mut Vec<Entity>){
       entities.retain(|entity| entity.get_component::<Radius>().unwrap().value > 1.0e-6);
    }
    
    pub fn create_new_grains(entities: &mut Vec<Entity>, params: &Params, init_strain: Matrix3<f64>){
        let mut new_grains:Vec<Entity> = Vec::new();
        
        for entity in entities.iter_mut(){
            let mut new_subgr:Vec<f64> = Vec::new();
           
            if let Some(subgrains) = entity.get_component::<Subgrains>(){
                let subgr = &subgrains.vector;
                for sg in subgr.iter(){
                    
                    let drive_force_st = sg.get_component::<DriveForce>().unwrap().value;
                    if  drive_force_st > 0.0 {
                        new_subgr.push(sg.get_component::<Radius>().unwrap().value);
                    }
                }
            }
            
            for subgr in new_subgr{
                
                let entity_volume_old = entity.get_component::<Volume>().unwrap().value;
                let entity_radius = entity.get_component_mut::<Radius>().unwrap();
                let subgr_volume = 4.0/3.0 * PI * subgr.powf(3.0);
                
                if entity_volume_old - subgr_volume > 0.0{
                    
                    let entity_volume = entity.get_component_mut::<Volume>().unwrap();
                    entity_volume.value -= subgr_volume;

                    let entity_radius = entity.get_component_mut::<Radius>().unwrap();
                    entity_radius.value = (3.0/4.0 / PI * entity_volume_old - subgr_volume).powf(1.0/3.0); 

                    let mut new_entity = entity!(1000, Radius, Volume, 
                    Rotation, TensorL, TensorD, TensorDe, TensorDin, TensorEps, 
                    TensorP, TensorSigma, TensorSigmaRate, 
                    BurgersVectors, BNMatrix, NormalsVectors, 
                    TauC, TauCRate, TauCHP, Tau, GammaRate, HVector, HMatrix,
                    DriveForce, AccumEnergyRate, AccumEnergy, FacetMobility, VelocityFacet);
                    let new_antity_rad = new_entity.get_component_mut::<Radius>().unwrap();
                    new_antity_rad.value = subgr;
                    let new_antity_vol = new_entity.get_component_mut::<Volume>().unwrap();
                    new_antity_vol.value = 4.0/3.0 * PI * subgr.powf(3.0);

                    new_grains.push(new_entity);
                }
            }
        }
        RotationSystem::initialize(&mut new_grains);
        StrainSystem::initialize(&mut new_grains, init_strain);
        //GeometrySystem::initialize_log(&mut new_grains, params.get_f64("gr_size"), params.get_f64("std_dev"));
        StressSystem::initialize(&mut new_grains, params.get_f64("c11"), params.get_f64("c12"), params.get_f64("c44"), params.get_f64("koef"));
        SlideSystem::initialize(&mut new_grains);
        DislocationSlidingSystem::initialize(&mut new_grains, params.get_f64("tau_c"));
        SubgrainSystem::initialize(&mut new_grains, params.get_i64("num_sg"), params.get_f64("r0"));
        RecrystallizationSystem::initialize_facet_mobility(&mut new_grains, params.get_f64("m0"), params.get_f64("Q"), params.get_f64("r"), params.get_f64("temp"));

        entities.extend(new_grains);
    }
}


                        
