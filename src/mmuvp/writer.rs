#![allow(dead_code)]

use std::{fs::OpenOptions, path::PathBuf, io::{BufWriter, Write}};

use crate::{entities::Entity, consts::FILE_OUTPUT_PATH, IntMeanSigma, IntMeanEps, TensorSigma, TensorEps};

use super::{geometry::components::Radius, recrystallization::components::AccumEnergy};

pub struct Writer;

impl Writer {
    pub fn write_intensity_to_file(entity: &Entity, step: i64, dt:f64){
        let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(PathBuf::from(FILE_OUTPUT_PATH).join("rvout.dat"))
        .expect("Ошибка открытия файла rvout.dat");
        //let file = File::create(FILE_OUTPUT_PATH.to_string() + "din.dat")?;
        let mut buf_writer = BufWriter::with_capacity(4 * (10 + 1 + 10 + 1) * 3, file);

        let int_sigma = entity.get_component::<IntMeanSigma>().unwrap().value;
        let int_eps = entity.get_component::<IntMeanEps>().unwrap().value;
        write!(buf_writer, "{:.4e}\t", int_eps)
            .expect("Ошибка записи интенсивности деформации в rvout.dat");
        write!(buf_writer, "{:.4e}\t", int_sigma)
            .expect("Ошибка записи интенсивности напряжения в rvout.dat");
        write!(buf_writer, "{}\t", dt * step as f64).expect("Ошибка записи времени в rvout.dat");

        writeln!(buf_writer).expect("Ошибка записи разделителя в rvout.dat");

        // Завершаем запись и проверяем наличие ошибок
        buf_writer
            .flush()
            .expect("Ошибка завершения записи в rvout.dat");
    }    

    pub fn write_mean_radius_to_file(entity: &Entity, step:i64, dt:f64){
        let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(PathBuf::from(FILE_OUTPUT_PATH).join("sizeout.dat"))
        .expect("Ошибка открытия файла sizeout.dat");
        //let file = File::create(FILE_OUTPUT_PATH.to_string() + "din.dat")?;
        let mut buf_writer = BufWriter::with_capacity(4 * (10 + 1 + 10 + 1) * 3, file);

        let mean_size = entity.get_component::<Radius>().unwrap().value;
        println!("{}",mean_size);
        write!(buf_writer, "{:.4e}\t", mean_size)
            .expect("Ошибка записи в sizeout.dat");
        write!(buf_writer, "{}\t", dt * step as f64).expect("Ошибка записи времени в sizeout.dat");

        writeln!(buf_writer).expect("Ошибка записи разделителя в sizeout.dat");

         // Завершаем запись и проверяем наличие ошибок
         buf_writer
         .flush()
         .expect("Ошибка завершения записи в rvout.dat");
    }

    pub fn write_nums_grain_to_file(entity: &Entity, entities: &[Entity], step:i64, dt:f64){
        let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(PathBuf::from(FILE_OUTPUT_PATH).join("ngout.dat"))
        .expect("Ошибка открытия файла ngout.dat");
        //let file = File::create(FILE_OUTPUT_PATH.to_string() + "din.dat")?;
        let mut buf_writer = BufWriter::with_capacity(4 * (10 + 1 + 10 + 1) * 3, file);

        let grain_num = entities.len();
        println!("{}",grain_num);
        write!(buf_writer, "{:.4e}\t", grain_num)
            .expect("Ошибка записи в ngout.dat");
        write!(buf_writer, "{}\t", dt * step as f64).expect("Ошибка записи времени в ngout.dat");

        writeln!(buf_writer).expect("Ошибка записи разделителя в ngout.dat");

         // Завершаем запись и проверяем наличие ошибок
         buf_writer
         .flush()
         .expect("Ошибка завершения записи в rvout.dat");
    }

    pub fn write_est_grain_to_file(entity: &Entity, step:i64, dt:f64){
        let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(PathBuf::from(FILE_OUTPUT_PATH).join("estout.dat"))
        .expect("Ошибка открытия файла estout.dat");
        //let file = File::create(FILE_OUTPUT_PATH.to_string() + "din.dat")?;
        let mut buf_writer = BufWriter::with_capacity(4 * (10 + 1 + 10 + 1) * 3, file);

        let est = entity.get_component::<AccumEnergy>().unwrap().value;
        write!(buf_writer, "{:.4e}\t", est)
            .expect("Ошибка записи в estout.dat");
        write!(buf_writer, "{}\t", dt * step as f64).expect("Ошибка записи времени в estout.dat");

        writeln!(buf_writer).expect("Ошибка записи разделителя в estout.dat");

         // Завершаем запись и проверяем наличие ошибок
         buf_writer
         .flush()
         .expect("Ошибка завершения записи в rvout.dat");
    }

    pub fn write_tensor_sigma_to_file(entity: &Entity, step:i64, dt:f64){
        let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(PathBuf::from(FILE_OUTPUT_PATH).join("sigmaout.dat"))
        .expect("Ошибка открытия файла sigmaout.dat");
        //let file = File::create(FILE_OUTPUT_PATH.to_string() + "din.dat")?;
        let mut buf_writer = BufWriter::with_capacity(4 * (10 + 1 + 10 + 1) * 3, file);

        
        let sigma = entity.get_component::<TensorSigma>().unwrap().tensor;
        for i in 0..3{
            for j in 0..3{
                write!(buf_writer, "{:.4e}\t", sigma[(i,j)])
                .expect("Ошибка записи в sigmaout.dat");
            }
        }
        write!(buf_writer, "{}\t", dt * step as f64).expect("Ошибка записи времени в sigmaout.dat");
        writeln!(buf_writer).expect("Ошибка записи разделителя в sigmaout.dat");


         // Завершаем запись и проверяем наличие ошибок
         buf_writer
         .flush()
         .expect("Ошибка завершения записи в sigmaout.dat");
    }

    pub fn write_tensor_eps_to_file(entity: &Entity, step:i64, dt:f64){
        let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(PathBuf::from(FILE_OUTPUT_PATH).join("epsout.dat"))
        .expect("Ошибка открытия файла epsout.dat");
        //let file = File::create(FILE_OUTPUT_PATH.to_string() + "din.dat")?;
        let mut buf_writer = BufWriter::with_capacity(4 * (10 + 1 + 10 + 1) * 3, file);

        
        let eps = entity.get_component::<TensorEps>().unwrap().tensor;
        for i in 0..3{
            for j in 0..3{
                write!(buf_writer, "{:.4e}\t", eps[(i,j)])
                .expect("Ошибка записи в epsout.dat");
            }
        }
        write!(buf_writer, "{}\t", dt * step as f64).expect("Ошибка записи времени в epsout.dat");
        writeln!(buf_writer).expect("Ошибка записи разделителя в epsout.dat");


         // Завершаем запись и проверяем наличие ошибок
         buf_writer
         .flush()
         .expect("Ошибка завершения записи в epsout.dat");
    }

    pub fn write_buff(entity: &Entity, entities: &[Entity], step:i64, dt:f64){
        let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(PathBuf::from(FILE_OUTPUT_PATH).join("buffout.dat"))
        .expect("Ошибка открытия файла epsout.dat");
        //let file = File::create(FILE_OUTPUT_PATH.to_string() + "din.dat")?;
        let mut buf_writer = BufWriter::with_capacity(4 * (10 + 1 + 10 + 1) * 3, file);

        let mut radius = 0.0;
        for entity in entities.iter(){
            if entity.id == 1000 {
                radius = entity.get_component::<Radius>().unwrap().value;
                break;
            }
        }
        if radius > 0.0 {
            write!(buf_writer, "{}\t", radius).expect("Ошибка записи значения в epsout.dat");
            write!(buf_writer, "{}\t", dt * step as f64).expect("Ошибка записи времени в epsout.dat");
            writeln!(buf_writer).expect("Ошибка записи разделителя в epsout.dat");
            // Завершаем запись и проверяем наличие ошибок
            buf_writer
            .flush()
            .expect("Ошибка завершения записи в epsout.dat");
            println!("Check!");
        }
        
        
    }
}