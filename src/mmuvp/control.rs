use std::{path::PathBuf, io::{self, BufRead}, fs::File};

use nalgebra::Matrix3;

use crate::consts::FILE_INPUT_PATH;

pub fn read_grad_v_from_file_with_6_comp(
    time_vector: &mut Vec<f64>,
    matrix_vector: &mut Vec<Matrix3<f64>>,
) {
    let file = File::open(PathBuf::from(FILE_INPUT_PATH).join("grad_v.input"))
        .expect("Не удалось открыть файл grad_v.input");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Потеряна строка в файле grad_v.input");
        let mut values = line.split_whitespace();

        let time: f64 = values
            .next()
            .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Missing time value"))
            .unwrap()
            .parse()
            .unwrap();

        let mut components = [0.0; 6];
        for component in components.iter_mut() {
            *component = values
                .next()
                .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Missing tensor component"))
                .unwrap()
                .parse()
                .unwrap();
        }

        let grad_v: Matrix3<f64> = Matrix3::new(
            components[0], components[1], components[2],
            components[1], components[3], components[4],
            components[2], components[4], components[5],
        );

        time_vector.push(time*10.0);
        matrix_vector.push(grad_v/10.0);
    }
}