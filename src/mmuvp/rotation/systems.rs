#![allow(dead_code)]
use std::{f64::consts::PI, path::PathBuf, fs::OpenOptions, io::{BufWriter, Write}};

use nalgebra::{Matrix3, Vector3};
use rand::Rng;

use crate::{entities::Entity, consts::FILE_OUTPUT_PATH};

use super::components::Rotation;


pub struct RotationSystem;

impl RotationSystem{
    pub fn initialize(entities: &mut [Entity]){
        for entity in entities.iter_mut(){
            if let Some(rotation) = entity.get_component_mut::<Rotation>(){
                if rotation.tensor == Matrix3::identity(){
                    rotation.tensor = get_uniform_distribution()
                }
            }
        }
    }
    pub fn write_pole_figure(entities: &[Entity]){
        let file100 = OpenOptions::new()
        .create(true)
        .append(true)
        .open(PathBuf::from(FILE_OUTPUT_PATH).join("pole_fig100.dat"))
        .expect("Ошибка открытия файла для записи poly_fig100.dat");
    //let file100 = File::create(FILE_OUTPUT_PATH.to_string() + "pole_fig100.dat")?;
    let mut buf_writer100 = BufWriter::with_capacity(4 * 25 * 3 * entities.len() + 4, file100);

    let file110 = OpenOptions::new()
        .create(true)
        .append(true)
        .open(PathBuf::from(FILE_OUTPUT_PATH).join("pole_fig110.dat"))
        .expect("Ошибка открытия файла для записи pole_fig110.dat");
    //let file110 = File::create(FILE_OUTPUT_PATH.to_string() + "pole_fig110.dat")?;
    let mut buf_writer110 = BufWriter::with_capacity(4 * 25 * 3 * entities.len() + 4, file110);

    let file111 = OpenOptions::new()
        .create(true)
        .append(true)
        .open(PathBuf::from(FILE_OUTPUT_PATH).join("pole_fig111.dat"))
        .expect("Ошибка открытия файла для записи pole_fig111.dat");
    //let file111 = File::create(FILE_OUTPUT_PATH.to_string() + "pole_fig111.dat")?;
    let mut buf_writer111 = BufWriter::with_capacity(4 * 25 * 3 * entities.len() + 4, file111);

    for entity in entities.iter(){
        if let Some(rotation) = entity.get_component::<Rotation>(){
            let rotation = rotation.tensor;
            let test_vector100 = Vector3::new(1.0, 0.0, 0.0);
            let rotation_vector100 =
                (test_vector100.normalize().transpose() * rotation).transpose();
            let test_vector110 = Vector3::new(1.0, 1.0, 0.0);
            let rotation_vector110 =
                (test_vector110.normalize().transpose() * rotation).transpose();
            let test_vector111 = Vector3::new(1.0, 1.0, 1.0);
            let rotation_vector111 =
                (test_vector111.normalize().transpose() * rotation).transpose();

            write!(
                buf_writer100,
                "{}\t{}\t{}\t",
                rotation_vector100.x, rotation_vector100.y, rotation_vector100.z
            )
            .expect("Ошибка записи полюсных фигур 100");
            write!(
                buf_writer110,
                "{}\t{}\t{}\t",
                rotation_vector110.x, rotation_vector110.y, rotation_vector110.z
            )
            .expect("Ошибка записи полюсных фигур 110");
            write!(
                buf_writer111,
                "{}\t{}\t{}\t",
                rotation_vector111.x, rotation_vector111.y, rotation_vector111.z
            )
            .expect("Ошибка записи полюсных фигур 111");
        }
    }

    writeln!(buf_writer100).unwrap();
    writeln!(buf_writer110).unwrap();
    writeln!(buf_writer111).unwrap();

    buf_writer100.flush().unwrap();
    buf_writer110.flush().unwrap();
    buf_writer111.flush().unwrap();

    }
    pub fn print_to_console(entities: &[Entity]){
        for entity in entities.iter(){
            println!("TensorO for grain №{}", entity.id);
            if let Some(rotation) = entity.get_component::<Rotation>(){
                for i in 0..3 {
                    for j in 0..3 {
                        print!("{:.2} ", rotation.tensor[(i, j)]);
                    }
                    println!();
                }
            }
        }
    }
}


fn get_uniform_distribution()->Matrix3<f64>{
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(0.0..2.0 * PI);
    let b: f64 = rng.gen_range(-1.0..1.0);
    let b = b.acos();
    let g = rng.gen_range(0.0..2.0 * PI);
    let ca = a.cos();
    let sa = a.sin();
    let cb = b.cos();
    let sb = b.sin();
    let cg = g.cos();
    let sg = g.sin();

    Matrix3::new(
        ca * cb * cg - sa * sg,
        -cg * sa - ca * cb * sg,
        ca * sb,
        cb * cg * sa + ca * sg,
        ca * cg - cb * sa * sg,
        sa * sb,
        -cg * sb,
        sb * sg,
        cb,
    )
}

