#![allow(dead_code)]

use std::{fs::File, path::PathBuf, io::{BufReader, BufRead}};

use nalgebra::{Vector3, Matrix3};

use crate::{entities::Entity, consts::{FILE_INPUT_PATH, MEGA}, mmuvp::{slide_systems::components::NormalsVectors, geometry::components::Radius}, TensorSigma, TauCHP};

use super::components::{BurgersVectors, BNMatrix, TauC, Tau, GammaRate, Gamma, HVector, HMatrix, TauCRate};

pub struct SlideSystem;

impl SlideSystem {
    pub fn initialize(entities:&mut [Entity]){
        Self::initialize_b(entities);
        Self::initialize_n(entities);
        Self::initialize_bn(entities);
    }

    pub fn initialize_b(entities:&mut [Entity]){
        let file = File::open(PathBuf::from(FILE_INPUT_PATH).join("b.input")).expect("Ошибка открытия файла b.input");
        let reader = BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            let line = line.expect("Ошибка. Файл b.input заполнен неверно");
            let values: Vec<f64> = line
                .split_whitespace()
                .map(|s| s.parse::<f64>().expect("Ошибка перевода строки b.input в числа"))
                .collect();
            if values.len()!=3 {
                panic!("Ошибка. Количество элементов в b.input равно {}",values.len());
            }
            let vector = Vector3::new(values[0], values[1], values[2]).normalize(); 
            for entity in entities.iter_mut(){
                if let Some(vectors_b) = entity.get_component_mut::<BurgersVectors>(){
                    if index < 12 {
                        vectors_b.vectors.push(vector);
                        vectors_b.vectors.push(-vector);
                    }
                }
            }
        } 
    }

    pub fn initialize_n(entities:&mut [Entity]){
        let file = File::open(PathBuf::from(FILE_INPUT_PATH).join("n.input")).expect("Ошибка открытия файла n.input");
        let reader = BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            let line = line.expect("Ошибка. Файл n.input заполнен неверно");
            let values: Vec<f64> = line
                .split_whitespace()
                .map(|s| s.parse::<f64>().expect("Ошибка перевода строки n.input в числа"))
                .collect();
            if values.len()!=3 {
                panic!("Ошибка. Количество элементов в n.input равно {}",values.len());
            }
            let vector = Vector3::new(values[0], values[1], values[2]).normalize(); 
            for entity in entities.iter_mut(){
                if let Some(vectors_n) = entity.get_component_mut::<NormalsVectors>(){
                    if index < 12 {
                        vectors_n.vectors.push(vector);
                        vectors_n.vectors.push(vector);
                    }
                }
            }
        } 
    }

    pub fn initialize_bn(entities:&mut [Entity]){
        for entity in entities.iter_mut(){
            let vectors_b = entity.get_component::<BurgersVectors>().unwrap().vectors.clone();
            let vectors_n = entity.get_component::<NormalsVectors>().unwrap().vectors.clone();

            if let Some(bn_matrix) = entity.get_component_mut::<BNMatrix>(){
                if bn_matrix.tensors.len() == 0{
                    for index in 0..24{
                        let b = vectors_b[index];
                        let n = vectors_n[index];
                        let mut bn_new:Matrix3<f64> = Matrix3::zeros();
                        for (i,bi) in b.iter().enumerate(){
                            for (j,nj) in n.iter().enumerate(){
                                bn_new[(i,j)]=bi*nj;
                            }
                        }
                        bn_matrix.tensors.push(bn_new);
                    }
                }
            }
            
        }
    }

    pub fn print_to_console_b(entities:&[Entity]){
        for entity in entities.iter(){
            if let Some(vector_b ) = entity.get_component::<BurgersVectors>(){
                let b = &vector_b.vectors;
                for bi in b{
                    println!("BurgersVectors of grain № {}: \t{}\t{}\t{}",entity.id, bi.x, bi.y, bi.z);
                }
            }
        }
    }

    pub fn print_to_console_n(entities:&[Entity]){
        for entity in entities.iter(){
            if let Some(vector_n ) = entity.get_component::<NormalsVectors>(){
                let n = &vector_n.vectors;
                for ni in n{
                    println!("NormalsVectors of grain № {}: \t{}\t{}\t{}",entity.id, ni.x, ni.y, ni.z);
                }
            }
        }
    }
}

pub struct DislocationSlidingSystem;

impl DislocationSlidingSystem {
    pub fn initialize(entities: &mut [Entity], tauc:f64){
        for entity in entities.iter_mut(){
            if let Some(tauc_component) = entity.get_component_mut::<TauC>(){
                let tauc_values = &mut tauc_component.values;
                for tauci in tauc_values.iter_mut() {
                    *tauci = tauc / MEGA;//MPa
                }
            }
        }
    }

    pub fn calc_tau(entities: &mut [Entity]){
        for entity in entities.iter_mut(){
            let bn = &entity.get_component::<BNMatrix>().unwrap().tensors.clone();
            let sigma = &entity.get_component::<TensorSigma>().unwrap().tensor.clone();
            if let Some(tau_component) = entity.get_component_mut::<Tau>(){
                let tau_values = &mut tau_component.values;
                for index in 0..24{
                    let tauu=bn[index].dot(sigma);
                    tau_values[index] = tauu;
                }
            }
        }
    }

    pub fn calc_gamma_rate(
        entities: &mut [Entity],
        gamma_0:f64,
        m:f64,
    ){
        for entity in entities.iter_mut(){
            let tau = entity.get_component::<Tau>().unwrap().values.clone();
            let mut tauc = entity.get_component::<TauC>().unwrap().values.clone();
            if let Some(tauc_hp) = entity.get_component::<TauCHP>(){
                let tauc_hp_values = &tauc_hp.values;
                for index in 0..24{
                    tauc[index]+=tauc_hp_values[index];
                }
            }
            if let Some(gamma_rate) = entity.get_component_mut::<GammaRate>(){
                let gamma_rate_values = &mut gamma_rate.values;
                for index in 0..24{
                    let ratio = tau[index]/tauc[index];
                    let gamma_rate_new = if ratio>1.0{
                        gamma_0 * ratio.powf(m)
                    } else {
                        0.0
                    };
                    gamma_rate_values[index] = gamma_rate_new;
                }
            }
            
        }
    }

    pub fn calc_gamma(
        entities: &mut [Entity],
        dt:f64
    ){
        for entity in entities.iter_mut(){
            let gamma_rate = &entity.get_component::<GammaRate>().unwrap().values.clone();
            if let Some(gamma) = entity.get_component_mut::<Gamma>(){
                let gamma_values = &mut gamma.values;
                for index in 0..24{
                    let old_gamma = gamma_values[index];
                    let new_gamma = old_gamma + gamma_rate[index] * dt;
                    gamma_values[index] = new_gamma;
                }
            }
        }
    }

    pub fn print_to_console_tauc(entities: &[Entity]){
        for entity in entities.iter(){
            if let Some(tauc_component) = entity.get_component::<TauC>(){
                let tauc_values = &tauc_component.values;
                for tauci in tauc_values.iter(){
                    println!("TauC of grain №{}:\t{}",entity.id,*tauci);
                }
            }
        }
    }

    pub fn print_to_console_tau(entities: &[Entity]){
        for entity in entities.iter(){
            if let Some(tau_component) = entity.get_component::<Tau>(){
                let tau_values = &tau_component.values;
                for taui in tau_values.iter(){
                    println!("Tau of grain №{}:\t{}",entity.id,*taui);
                }
            }
        }
    }
}


pub struct HarderingSystem;

impl HarderingSystem {
    pub fn calc_h_vector(
        entities: &mut [Entity],
        tau_sat:f64,
        h0:f64,
        a:f64,
    ){
        for entity in entities.iter_mut(){
            let tau_c = entity.get_component::<TauC>().unwrap().values.clone();
            if let Some(hvector) = entity.get_component_mut::<HVector>(){
                let hvector_value = &mut hvector.vector;
                for index in 0..24{
                    let tau_sat = tau_sat/MEGA;//MPa
                    let ratio = tau_c[index]/tau_sat;
                    let absol = (1.0-ratio).abs();
                    let pow_absol = absol.powf(a);
                    let value = h0*pow_absol;
                    hvector_value[index] = value;
                }
            }
        }
    }

    pub fn calc_h_matrix(entities: &mut [Entity], qlat:f64){
        for entity in entities.iter_mut(){
            let hvector = entity.get_component::<HVector>().unwrap().vector.clone();
            if let Some(hmatrix) = entity.get_component_mut::<HMatrix>(){
                let hmatrix_value = &mut hmatrix.matrix;
                for index_i in 0..24{
                    for index_j in 0..24{
                        let multiply = if index_i==index_j{1.0} else{qlat};
                        let value = hvector[index_j]*multiply;
                        hmatrix_value[index_i][index_j] = value;
                    }
                }
            }
        }
    }

    pub fn calc_tauc_rate(entities: &mut [Entity]){
        for entity in entities.iter_mut(){
            let gamma_rate = entity.get_component::<GammaRate>().unwrap().values.clone();
            let hmatrix = entity.get_component::<HMatrix>().unwrap().matrix.clone();
            if let Some(tauc_rate) = entity.get_component_mut::<TauCRate>(){
                for index_i in 0..24{
                    let mut summ=0.0;
                    for index_j in 0..24{
                         summ += hmatrix[index_i][index_j]*gamma_rate[index_j];
                    }
                    if gamma_rate[index_i].abs() > 1.0e-7{
                        tauc_rate.values[index_i] = summ;
                    } else {
                        tauc_rate.values[index_i] = 0.0;
                    }
                }
            }
        }
    }

    pub fn calc_tauc(entities: &mut [Entity], dt:f64){
        for entity in entities.iter_mut(){
            let tauc_rate = entity.get_component::<TauCRate>().unwrap().values.clone();
            if let Some(tauc) = entity.get_component_mut::<TauC>(){
                for index in 0..24{
                    let tauc_old = tauc.values[index];
                    let tauc_new = tauc_rate[index] * dt;
                    tauc.values[index] = tauc_old + tauc_new;
                }
            }
        }
    }

    pub fn calc_tauc_hp(entities: &mut [Entity], b:f64, k_y:f64){
        for entity in entities.iter_mut(){
            let gr_size = entity.get_component::<Radius>().unwrap().value;
            let addition_hp = k_y*(b / gr_size).sqrt() / MEGA;
            if let Some(tauc_hp) = entity.get_component_mut::<TauCHP>(){
                for index in 0..24{
                    tauc_hp.values[index] = addition_hp;//MPa
                }
            }
        }
    }
}