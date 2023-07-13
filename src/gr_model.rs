use std::{collections::HashMap, mem};

use cgmath::Vector3;

use crate::{dtw::DTW, dtw3d::DTW3D};

pub struct GRModel {
    train_data: Vec<DataRecord>,
    test_data: Vec<DataRecord>,
    dba_model: HashMap<u32, Vec<Vector3<f64>>>,
}

#[derive(Debug, Clone)]
pub struct DataRecord {
    pub id: u32,
    pub gesture: u32,
    pub sequence: Vec<Vector3<f64>>,
}

impl GRModel {
    pub fn new(train_data: Vec<DataRecord>, test_data: Vec<DataRecord>) -> GRModel {
        GRModel {
            train_data,
            test_data,
            dba_model: HashMap::<u32, Vec<Vector3<f64>>>::new(),
        }
    }

    // fn combine_sequences(seq1: &Vec<Vector3<f64>>, seq2: &Vec<Vector3<f64>>) -> Vec<Vector3<f64>> {
    //     assert_eq!(seq1.len(), seq2.len());

    //     let mut combined_sequence = Vec::<Vector3<f64>>::with_capacity(seq1.len());
    //     for (x, y) in iter::zip(seq1, seq2) {
    //         combined_sequence.push((x + y) / 2.0);
    //     }
    //     combined_sequence
    // }

    fn update_sequences(&mut self, gesture: u32, sequence: Vec<Vector3<f64>>) {
        println!("{:#?}", sequence);
        self.dba_model.insert(gesture, sequence);
    }

    pub fn train(&mut self) {
        for data_point in &self.train_data.clone() {
            self.update_sequences(data_point.gesture, data_point.sequence.clone());
        }
        println!("{:#?}", self.dba_model);
    }

    pub fn test(&self) {
        let mut correct_guesses = 0;
        let mut guesses = 0;

        for (i, data_point) in (&self.test_data).iter().enumerate() {
            if i % 5 == 0 {
                println!("Completed 5");
            }
            let mut lowest_cost_gesture = 0;
            let mut dtw = DTW3D::default();
            let mut lowest_cost = f64::MAX;
            for i in self.dba_model.keys() {
                let gesture = *i;
                dtw = DTW3D::new(self.dba_model[i].clone(), data_point.sequence.clone());
                if dtw.get_cost() < lowest_cost {
                    lowest_cost = dtw.get_cost();
                    lowest_cost_gesture = gesture;
                }
            }
            if lowest_cost_gesture == data_point.gesture {
                dtw.print_dtw_path();
                correct_guesses += 1;
            }
            guesses += 1;
        }

        println!("Accuracy: {}", (correct_guesses as f64 / guesses as f64));
    }
}
