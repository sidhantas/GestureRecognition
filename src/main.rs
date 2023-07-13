use cgmath::Vector3;
use serde::{Deserialize, Deserializer};
use std::env;
use std::error::Error;
use std::fmt::Debug;
use std::mem;

use splines::{Interpolation, Key, Spline};

pub mod dtw;
pub mod dtw3d;
pub mod gr_model;

use gr_model::DataRecord;
use gr_model::GRModel;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let invalid_input_error = "Invalid Input\nCorrect Usage: ./gesture_map [input.csv]";
    let filename = match args.get(1) {
        Some(f) => f,
        None => return Err(invalid_input_error.into()),
    };

    let mut rdr = match csv::Reader::from_path(filename) {
        Ok(file) => file,
        Err(e) => return Err(e.into()),
    };
    let mut data = Vec::<DataRecord>::new();

    for res in rdr.deserialize() {
        let rec: Record = res.unwrap();

        data.push(DataRecord {
            id: rec.id,
            gesture: rec.gesture,
            sequence: zip3(rec.x, rec.y, rec.z),
        })
    }

    let (train, test) = train_test_split(data, 0.8);
    let mut gr = GRModel::new(train, test);
    gr.train();
    gr.test();
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Record {
    id: u32,
    user: u32,
    gesture: u32,
    #[serde(deserialize_with = "vec_deserializer")]
    x: Vec<f64>,
    #[serde(deserialize_with = "vec_deserializer")]
    y: Vec<f64>,
    #[serde(deserialize_with = "vec_deserializer")]
    z: Vec<f64>,
}

fn zip3<T: Copy + Debug>(x: Vec<T>, y: Vec<T>, z: Vec<T>) -> Vec<Vector3<T>> {
    assert_eq!(x.len(), y.len());
    assert_eq!(x.len(), z.len());

    let mut v: Vec<Vector3<T>> = Vec::with_capacity(x.len());

    for i in 0..x.len() {
        v.push(Vector3 {
            x: x[i],
            y: y[i],
            z: z[i],
        });
    }
    v
}

fn vec_deserializer<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let str_sequence = String::deserialize(deserializer)?;
    Ok(str_sequence
        .replace(&['[', ']'][..], "")
        .split(",")
        .map(|item| item.trim().parse().unwrap())
        .collect())
}

fn train_test_split<T: Clone>(v: Vec<T>, train_size: f64) -> (Vec<T>, Vec<T>) {
    let training_capacitiy = (v.len() as f64 * train_size) as usize;
    let mut train_data = Vec::<T>::with_capacity(training_capacitiy);
    for i in 0..train_data.capacity() {
        train_data.push(v[i].clone());
    }
    train_data.clone_from_slice(&v[0..training_capacitiy]);
    let mut test_data = Vec::<T>::with_capacity(v.len() - train_data.capacity());
    for i in train_data.capacity()..v.len() {
        test_data.push(v[i].clone());
    }

    (train_data, test_data)
}
