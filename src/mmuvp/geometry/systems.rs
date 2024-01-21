#![allow(dead_code)]

use std::f64::consts::PI;

use rand::{distributions::Distribution, Rng};
use statrs::distribution::LogNormal;

use crate::entities::Entity;

use super::components::{Radius, Volume};

pub struct GeometrySystem;

impl GeometrySystem {
    pub fn initialize_log(entities: &mut [Entity], mean: f64, std_dev: f64){
        if entities.len() >1 {
            for entity in entities.iter_mut(){
                if let Some(radius) = entity.get_component_mut::<Radius>(){
                    let value = generate_lognormal_random_number(mean, std_dev);
                    radius.value = value;
                }
    
                let r = entity.get_component::<Radius>().unwrap().value;
                if let Some(volume) = entity.get_component_mut::<Volume>(){
                    let value = 4.0/3.0 * PI * r.powf(3.0);
                    volume.value = value;
                }
            }
        } else {
            for entity in entities.iter_mut(){
                if let Some(radius) = entity.get_component_mut::<Radius>(){
                    let value = mean;
                    radius.value = value;
                }
    
                let r = entity.get_component::<Radius>().unwrap().value;
                if let Some(volume) = entity.get_component_mut::<Volume>(){
                    let value = 4.0/3.0 * PI * r.powf(3.0);
                    volume.value = value;
                }
            }
        }
    }
    
    pub fn initialize_rey(entities: &mut [Entity], mean: f64){
        let mut buff_vec:Vec<f64> = vec![];

        get_distr_rayleigh(&mut buff_vec, entities.len(), mean);
        
        for (i, entity) in entities.iter_mut().enumerate(){
            if let Some(radius) = entity.get_component_mut::<Radius>(){
                let value = buff_vec[i];
                radius.value = value;
            }

            let r = entity.get_component::<Radius>().unwrap().value;
            if let Some(volume) = entity.get_component_mut::<Volume>(){
                let value = 4.0/3.0 * PI * r.powf(3.0);
                volume.value = value;
            }
        }
    }

    pub fn calc_mean_size(output_entity: &mut Entity, entities: & [Entity]){
        let mut summ = 0.0;
        let mut summvol = 0.0;
        for entity in entities.iter(){
            let volume = entity.get_component::<Volume>().unwrap().value;
            let radius = entity.get_component::<Radius>().unwrap().value;
            summvol += volume;
            summ+= radius*volume;
        }
        summ /= summvol;

        if let Some(mean_radius) = output_entity.get_component_mut::<Radius>(){
            mean_radius.value = summ;
        }
    }

    pub fn print_to_console_radius(entities: & [Entity]){
        for entity in entities.iter(){
            if let Some(radius) = entity.get_component::<Radius>(){
                println!("Radius of grain №{}:\t{}", entity.id, radius.value);
            }
        }
    }

    pub fn print_to_console_volume(entities: & [Entity]){
        for entity in entities.iter(){
            if let Some(volume) = entity.get_component::<Volume>(){
                println!("Volume of grain №{}:\t{}", entity.id, volume.value);
            }
        }
    }
}

fn generate_lognormal_random_number(mean: f64, std_dev: f64) -> f64 {
    let disp = std_dev.powf(2.0);
    let mean2= mean.powf(2.0);
    let location = (mean2/(mean2+disp).sqrt()).ln();
    let scale = (1.0+(disp/mean2)).ln();
    let mut rng = rand::thread_rng();
    let lognormal = LogNormal::new(location,scale).unwrap();
    lognormal.sample(&mut rng)
}

pub fn get_distr_rayleigh(distr: &mut Vec<f64>, num: usize, r0: f64) {
    let mut ev_dist = Vec::new();
    let mut distr_den = Vec::new();
    let mut rando;
    let mut max: f64;
    let mut nnum = num;
    let mut rng = rand::thread_rng();
    loop {
        nnum += num / 2;
        ev_dist.clear();
        distr_den.clear();
        distr.clear();
        max = 0.0;

        for _ in 0..nnum {
            rando = rng.gen_range(0.0..(10.0 * r0));
            ev_dist.push(rando);
        }

        #[allow(clippy::needless_range_loop)]
        for i in 0..nnum {
            rando = distr_dens_rayleigh(r0, ev_dist[i]);
            distr_den.push(rando);
            if rando > max {
                max = rando;
            }
        }

        for i in 0..nnum {
            if distr_den[i] / max >= rng.gen_range(0.0..1.0) {
                distr.push(ev_dist[i]);
            }
        }

        if distr.len() >= num {
            break;
        }
    }

    nnum = distr.len() - num;
    distr.drain(..nnum);
}

pub fn distr_dens_rayleigh(r0: f64, x: f64) -> f64 {
    let sigma = r0 * f64::sqrt(2.0 / (2.0 * f64::asin(1.0)));
    if x >= 0.0 {
        x / (sigma * sigma) * f64::exp(-(x * x) / (2.0 * sigma * sigma))
    } else {
        0.0
    }
}