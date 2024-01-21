#![allow(unused_variables)]
use std::time::Instant;

use base_fn::clear_output_folder;
use mmuvp::{
    standart_deformation::*,
    rotation::{
        systems::RotationSystem, 
        components::Rotation}, 
    params::{
        components::Params, 
        systems::ParamsSystem}, 
    ep_deformation::{
        components::*, 
        systems::*
    }, 
    geometry::{
        systems::GeometrySystem, 
        components::{Radius, Volume}
    }, 
    slide_systems::{
        components::*, 
        systems::*
    }, recrystallization::{components::{Subgrains, DriveForce, AccumEnergyRate, AccumEnergy, FacetMobility}, systems::{SubgrainSystem, RecrystallizationSystem}}, control::read_grad_v_from_file_with_6_comp
};
use nalgebra::Matrix3;

use crate::{entities::Entity, mmuvp::writer::Writer};

mod entities;
mod mmuvp;
mod consts;
mod base_fn;


fn main() {
    clear_output_folder();
    let mut params = Params::default();
    ParamsSystem::from_file(&mut params);
    let dt = params.get_f64("dt");

    let init_strain = uniaxial_tension(1.0e-3);
 
    
    let mut entities:Vec<Entity> = vec![];
    let mut polycrystal_entities = entity!(0, TensorEps, TensorSigma, IntMeanSigma, IntMeanEps, AccumEnergy, Radius);

    for grain_id in 0..params.get_i64("grain_num"){
        let grain_id = grain_id as usize;
        entities.push(entity!(grain_id, Radius, Volume, 
            Rotation, TensorL, TensorD, TensorDe, TensorDin, TensorEps, 
            TensorP, TensorSigma, TensorSigmaRate, 
            BurgersVectors, BNMatrix, NormalsVectors, 
            TauC, TauCRate, TauCHP, Tau, GammaRate, HVector, HMatrix, 
            Subgrains, DriveForce, AccumEnergyRate, AccumEnergy, FacetMobility));
    } 
    
    RotationSystem::initialize(&mut entities);
    StrainSystem::initialize(&mut entities, init_strain);
    GeometrySystem::initialize_log(&mut entities, params.get_f64("gr_size"), params.get_f64("std_dev"));
    StressSystem::initialize(&mut entities, params.get_f64("c11"), params.get_f64("c12"), params.get_f64("c44"), params.get_f64("koef"));
    SlideSystem::initialize(&mut entities);
    DislocationSlidingSystem::initialize(&mut entities, params.get_f64("tau_c"));
    SubgrainSystem::initialize(&mut entities, 2000, params.get_f64("r0"));
    RecrystallizationSystem::initialize_facet_mobility(&mut entities, params.get_f64("m0"), params.get_f64("Q"), params.get_f64("r"), params.get_f64("temp"));
    let mut grad_v:Vec<Matrix3<f64>> = Vec::new();
    let mut times:Vec<f64> = Vec::new();
    read_grad_v_from_file_with_6_comp(&mut times, &mut grad_v);
    

    Writer::write_intensity_to_file(&polycrystal_entities, 0, dt);
    let time = Instant::now();
    for step in 1..params.get_i64("steps_num")+1{
        
        if (step as f64) * dt >= *(times.first().unwrap()){
            StrainSystem::initialize(&mut entities, *grad_v.first().unwrap());
            times.remove(0);
            grad_v.remove(0);
        }

        DislocationSlidingSystem::calc_tau(&mut entities);
        DislocationSlidingSystem::calc_gamma_rate(&mut entities, params.get_f64("gamma_0"), params.get_f64("m"));
        HarderingSystem::calc_h_vector(&mut entities, params.get_f64("tau_sat"), params.get_f64("h0"), params.get_f64("a"));
        HarderingSystem::calc_h_matrix(&mut entities, params.get_f64("qlat"));
        HarderingSystem::calc_tauc_rate(&mut entities);
        HarderingSystem::calc_tauc(&mut entities, dt);
        HarderingSystem::calc_tauc_hp(&mut entities,params.get_f64("b"), params.get_f64("k_y"));
        StrainSystem::calc_din(&mut entities);
        StrainSystem::calc_de(&mut entities);
        StrainSystem::calc_eps(&mut entities, dt);
        StressSystem::calc_hooke_law(&mut entities);
        StressSystem::calc_sigma(&mut entities, dt);
        RecrystallizationSystem::calc_accum_energy_rate(&mut entities, params.get_f64("alfa"));
        RecrystallizationSystem::calc_accum_energy(&mut entities, dt);
        RecrystallizationSystem::calc_mean_accum_energy(&mut polycrystal_entities, &entities);
        RecrystallizationSystem::calc_drive_force(&mut entities, &polycrystal_entities, params.get_f64("egb"));
        RecrystallizationSystem::calc_velocity_facet(&mut entities);
        RecrystallizationSystem::calc_radius(&mut entities, dt);
        RecrystallizationSystem::remove_grain(&mut entities);
        RecrystallizationSystem::create_new_grains(&mut entities, &params, init_strain);
        
        let current_time = time.elapsed();
        if step % params.get_i64("write_step") == 0{
            //println!(" Value:{}", entities[0].get_component::<Volume>().unwrap().value);
            StressSystem::calc_mean_sigma(&mut polycrystal_entities, &entities);
            StressSystem::calc_int_mean_sigma(&mut polycrystal_entities);
            StrainSystem::calc_mean_eps(&mut polycrystal_entities, &entities);
            StrainSystem::calc_int_mean_eps(&mut polycrystal_entities);
            Writer::write_intensity_to_file(&polycrystal_entities, step, dt);
            GeometrySystem::calc_mean_size(&mut polycrystal_entities, &entities);
            Writer::write_mean_radius_to_file(&polycrystal_entities, step, dt);
            //Writer::write_nums_grain_to_file(&polycrystal_entities, &entities, step, dt);
            //Writer::write_buff(&polycrystal_entities, &entities, step, dt);
            Writer::write_est_grain_to_file(&polycrystal_entities, step, dt);
            //Writer::write_tensor_sigma_to_file(&polycrystal_entities, step, dt);
            //Writer::write_tensor_eps_to_file(&polycrystal_entities, step, dt);
            StrainSystem::print_to_console_eps_single(&polycrystal_entities);
            StressSystem::print_to_console_sigma_single(&polycrystal_entities);
        }
    }
}
