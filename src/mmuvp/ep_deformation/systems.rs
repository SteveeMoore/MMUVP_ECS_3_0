#![allow(dead_code)]

use nalgebra::{Matrix3, Vector6, Matrix6};

use crate::{entities::Entity, mmuvp::{rotation::components::Rotation, slide_systems::components::{BNMatrix, GammaRate}, geometry::components::Volume}, consts::{MEGA, GIGA}};

use super::components::*;

pub struct StrainSystem;

impl StrainSystem{
    pub fn initialize(entities: &mut [Entity], init_strain:Matrix3<f64>){
        for entity in entities.iter_mut(){
            let tensor_o = entity.get_component::<Rotation>().unwrap().tensor;

            if let Some(tensor_l) = entity.get_component_mut::<TensorL>(){
                let new_tensor = tensor_o.transpose() * init_strain * tensor_o;
                tensor_l.tensor = new_tensor;
            }

            let tensor_l = entity.get_component::<TensorL>().unwrap().tensor;
            if let Some(tensor_d) = entity.get_component_mut::<TensorD>(){
                let new_tensor = (tensor_l+tensor_l.transpose())/2.0;
                tensor_d.tensor = new_tensor;
            }
        }

    }

    pub fn initialize_single(entity: &mut Entity, init_strain:Matrix3<f64>){
        let tensor_o = entity.get_component::<Rotation>().unwrap().tensor;
        let new_tensor = tensor_o.transpose() * init_strain * tensor_o;

        if let Some(tensor_l) = entity.get_component_mut::<TensorL>(){
            tensor_l.tensor = new_tensor;
        }

        if let Some(tensor_d) = entity.get_component_mut::<TensorD>(){
            let new_tensor = (new_tensor+new_tensor.transpose())/2.0;
            tensor_d.tensor = new_tensor;
        }
    }



    pub fn calc_eps(entities: &mut [Entity], dt:f64){
        for entity in entities.iter_mut(){
            Self::calc_eps_single(entity, dt);
        }
    }

    pub fn calc_eps_single(entity: &mut Entity, dt:f64){
        let tensor_d = entity.get_component::<TensorD>().unwrap().tensor;
        if let Some(tensor_eps) = entity.get_component_mut::<TensorEps>(){
            let tensor_eps_old = tensor_eps.tensor;
            let tensor_eps_new = tensor_eps_old+tensor_d*dt;
            tensor_eps.tensor = tensor_eps_new;
        }
    }


    pub fn calc_de(entities: &mut [Entity]){
        for entity in entities.iter_mut(){
            let tensor_d = entity.get_component::<TensorD>().unwrap().tensor;
            let tensor_din = entity.get_component::<TensorDin>().unwrap().tensor;
            if let Some(tensor_de) = entity.get_component_mut::<TensorDe>(){
                let tensor_de_new = tensor_d-tensor_din;
                tensor_de.tensor = tensor_de_new;
            }
        }
    }
    
    pub fn calc_din(entities: &mut [Entity]){
        for entity in entities.iter_mut(){
            let gamma_rate = entity.get_component::<GammaRate>().unwrap().values.clone();
            let bn = entity.get_component::<BNMatrix>().unwrap().tensors.clone();
            if let Some(din) = entity.get_component_mut::<TensorDin>(){
                let mut din_new = Matrix3::zeros();
                for index in 0..24{
                    let gamma_rate_i = gamma_rate[index];
                    let bni = bn[index];
                    let term = gamma_rate_i*bni;
                    din_new+=term;
                }
                din.tensor = din_new;
            }
        }
    }

    pub fn calc_mean_eps(output_entity:&mut Entity, entities: & [Entity]){
        let mut summvol=0.0;
        let mut mean_eps:Matrix3<f64> = Matrix3::zeros();
        if let Some(eps_output) = output_entity.get_component_mut::<TensorEps>(){
            for entity in entities.iter(){
                let eps = entity.get_component::<TensorEps>().unwrap().tensor.clone();
                let o = entity.get_component::<Rotation>().unwrap().tensor.clone();
                let volume = entity.get_component::<Volume>().unwrap().value;
                summvol+=volume;
                let mean_eps_i = o*eps*o.transpose();
                mean_eps += mean_eps_i*volume;
            }   
            mean_eps/=summvol;
            eps_output.tensor = mean_eps;
        }
    }

    pub fn calc_int_mean_eps(entity:&mut Entity){
        let eps = entity.get_component::<TensorEps>().unwrap().tensor.clone();
        if let Some(intsigma) = entity.get_component_mut::<IntMeanEps>(){
            let value = (eps.dot(&eps)*2.0/3.0).sqrt();
            intsigma.value = value;
        }
    }

    pub fn print_to_console_d(entities: & [Entity]){
        for entity in entities.iter(){
            println!("TensorD for grain №{}", entity.id);
            if let Some(tensor_l) = entity.get_component::<TensorD>(){
                for i in 0..3 {
                    for j in 0..3 {
                        print!("{:.2} ", tensor_l.tensor[(i, j)]);
                    }
                    println!();
                }
            }
        }
    }

    pub fn print_to_console_eps(entities: & [Entity]){
        for entity in entities.iter(){
            Self::print_to_console_eps_single(entity);
        }
    }

    pub fn print_to_console_eps_single(entity:&Entity){
        println!("TensorEps for grain №{}", entity.id);
            if let Some(tensor_eps) = entity.get_component::<TensorEps>(){
                for i in 0..3 {
                    for j in 0..3 {
                        print!("{:.2} ", tensor_eps.tensor[(i, j)]);
                    }
                    println!();
                }
            }
    }

    pub fn print_to_console_l(entities: & [Entity]){
        for entity in entities.iter(){
            println!("TensorL for grain №{}", entity.id);
            if let Some(tensor_l) = entity.get_component::<TensorL>(){
                for i in 0..3 {
                    for j in 0..3 {
                        print!("{:.2} ", tensor_l.tensor[(i, j)]);
                    }
                    println!();
                }
            }
        }
    }
}

pub struct StressSystem;
impl StressSystem {
    pub fn initialize(entities: &mut [Entity], 
        c11: f64,
        c12: f64,
        c44: f64,
        koef:f64
    ){
        for entity in entities.iter_mut(){
            let c11 = c11 / MEGA;//MPa
            let c12 = c12 / MEGA;//MPa
            let c44 = c44 / MEGA;//MPa
            if let Some(tensor_p) = entity.get_component_mut::<TensorP>(){
                if tensor_p.tensor == Matrix6::zeros(){
                    let new_tensor_p = Matrix6::new(
                        c11, c12, c12, 0.0, 0.0, 0.0, c12, c11, c12, 0.0, 0.0, 0.0, c12, c12, c11, 0.0, 0.0,
                        0.0, 0.0, 0.0, 0.0, c44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, c44, 0.0, 0.0, 0.0, 0.0, 0.0,
                        0.0, c44,
                    )*koef;
                    tensor_p.tensor = new_tensor_p;
                }
            }
        }
    }

    pub fn calc_hooke_law(entities: &mut [Entity]){
        for entity in entities.iter_mut(){
            let p = entity.get_component::<TensorP>().unwrap().tensor;
            let de = entity.get_component::<TensorDe>().unwrap().tensor;
            let vector_de = Vector6::new(
                de[(0, 0)],
                de[(1, 1)],
                de[(2, 2)],
                (de[(0, 1)] + de[(1, 0)]) / 2.0,
                (de[(0, 2)] + de[(2, 0)]) / 2.0,
                (de[(1, 2)] + de[(2, 1)]) / 2.0,
            );
            if let Some(tensor_sigma_rate) = entity.get_component_mut::<TensorSigmaRate>(){
                let vector_sigma_rate_new = p*vector_de;
                let tensor_sigma_rate_new = Matrix3::new(
                    vector_sigma_rate_new[0], vector_sigma_rate_new[3], vector_sigma_rate_new[4], 
                    vector_sigma_rate_new[3], vector_sigma_rate_new[1], vector_sigma_rate_new[5], 
                    vector_sigma_rate_new[4], vector_sigma_rate_new[5], vector_sigma_rate_new[2],
                );
                tensor_sigma_rate.tensor = tensor_sigma_rate_new;
            }
        }
    }

    pub fn calc_sigma( entities: &mut [Entity], dt:f64){
        for entity in entities.iter_mut(){
            let sigma_rate = entity.get_component::<TensorSigmaRate>().unwrap().tensor;
            if let Some(tensor_sigma) = entity.get_component_mut::<TensorSigma>(){
                let sigma_old = tensor_sigma.tensor;
                let sigma_new = sigma_old+sigma_rate*dt;
                tensor_sigma.tensor = sigma_new;
            }
        }
    }

    pub fn calc_mean_sigma(output_entity:&mut Entity, entities: & [Entity]){
        let mut summvol=0.0;
        let mut mean_sigma:Matrix3<f64> = Matrix3::zeros();
        if let Some(sigma_output) = output_entity.get_component_mut::<TensorSigma>(){
            for entity in entities.iter(){
                let sigma = entity.get_component::<TensorSigma>().unwrap().tensor.clone();
                let o = entity.get_component::<Rotation>().unwrap().tensor.clone();
                let volume = entity.get_component::<Volume>().unwrap().value;
                summvol+=volume;
                let mean_sigma_i = o*sigma*o.transpose();
                mean_sigma += mean_sigma_i*volume;
            }   
            mean_sigma/=summvol;
            sigma_output.tensor = mean_sigma;
        }
    }

    pub fn calc_int_mean_sigma(entity:&mut Entity){
        let sigma = entity.get_component::<TensorSigma>().unwrap().tensor.clone();
        if let Some(intsigma) = entity.get_component_mut::<IntMeanSigma>(){
            let value = (sigma.dot(&sigma)*3.0/2.0).sqrt();
            intsigma.value = value;
        }
    }
    
    pub fn print_to_console_p(entities: & [Entity]){
        for entity in entities.iter(){
            println!("TensorP for grain №{}", entity.id);
            if let Some(tensor_p) = entity.get_component::<TensorP>(){
                for i in 0..6 {
                    for j in 0..6 {
                        print!("{:.2} ", tensor_p.tensor[(i, j)]*MEGA/GIGA);//GPa
                    }
                    println!();
                }
            }
        }
    }

    pub fn print_to_console_sigma(entities: & [Entity]){
        for entity in entities.iter(){
            Self::print_to_console_sigma_single(entity);
        }
    }

    pub fn print_to_console_sigma_single(entity: & Entity){
        println!("TensorSigma for grain №{}", entity.id);
        if let Some(tensor_sigma) = entity.get_component::<TensorSigma>(){
            for i in 0..3 {
                for j in 0..3 {
                    print!("{:.2} ", tensor_sigma.tensor[(i, j)]);
                }
                println!();
            }
        }
    }
    pub fn print_to_console_sigma_rate(entities: & [Entity]){
        for entity in entities.iter(){
            println!("TensorSigmaRate for grain №{}", entity.id);
            if let Some(tensor_sigma_rate) = entity.get_component::<TensorSigmaRate>(){
                for i in 0..3 {
                    for j in 0..3 {
                        print!("{:.2} ", tensor_sigma_rate.tensor[(i, j)]);
                    }
                    println!();
                }
            }
        }
    }
}