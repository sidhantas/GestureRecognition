use cgmath::Vector3;
use colored::Colorize;
use std::fmt;
use std::mem;

use crate::dtw::DTW;

#[derive(Default, Clone)]
pub struct DTW3D {
    seq1: Vec<Vector3<f64>>,
    seq2: Vec<Vector3<f64>>,
    cost_matrix: Vec<Vec<f64>>,
    dtw_matrix: Vec<Vec<f64>>,
    aligned_sequence: Vec<Vector3<f64>>,
    dtw_cost: f64,
    normalized_dtw_cost: f64,
    dtw_path: Vec<(usize, usize)>,
}

impl DTW3D {
    fn cost(&self, seq1_point: &Vector3<f64>, seq2_point: &Vector3<f64>) -> f64 {
        (seq1_point.y - seq2_point.y).powi(2)
            + (seq1_point.x - seq2_point.x).powi(2)
            + (seq1_point.z - seq2_point.z).powi(2)
    }

    pub fn new(seq1: Vec<Vector3<f64>>, seq2: Vec<Vector3<f64>>) -> DTW3D {
        let mut d = DTW3D {
            seq1,
            seq2,
            ..Default::default()
        };
        d.create_cost_matrix();
        d.create_dtw_matrix();
        d.create_dtw_path();

        d
    }

    fn create_cost_matrix(&mut self) {
        self.cost_matrix = vec![vec![Default::default(); self.seq2.len()]; self.seq1.len()];

        for i in 0..self.cost_matrix.len() {
            for j in 0..self.cost_matrix[0].len() {
                self.cost_matrix[i][j] = self.cost(&self.seq1[i], &self.seq2[j]);
            }
        }
    }

    fn create_dtw_matrix(&mut self) {
        self.dtw_matrix = vec![vec![Default::default(); self.seq2.len() + 1]; self.seq1.len() + 1];

        for i in 0..self.dtw_matrix.len() {
            self.dtw_matrix[i][0] = f64::MAX;
        }

        for i in 0..self.dtw_matrix[0].len() {
            self.dtw_matrix[0][i] = f64::MAX;
        }

        self.dtw_matrix[0][0] = 0.0;

        for i in 1..self.dtw_matrix.len() {
            for j in 1..self.dtw_matrix[0].len() {
                self.dtw_matrix[i][j] = self.cost_matrix[i - 1][j - 1]
                    + vec![
                        self.dtw_matrix[i - 1][j],
                        self.dtw_matrix[i][j - 1],
                        self.dtw_matrix[i - 1][j - 1],
                    ]
                    .iter()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
            }
        }
        self.dtw_cost = *self.dtw_matrix.last().unwrap().last().unwrap();
        self.normalized_dtw_cost = self.dtw_cost / ((self.seq1.len() + self.seq2.len()) / 2) as f64;
    }

    fn create_dtw_path(&mut self) {
        let mut pos = (self.dtw_matrix.len() - 1, self.dtw_matrix[0].len() - 1);
        self.dtw_path.push(pos);
        while pos != (1, 1) {
            let x = pos.0;
            let y = pos.1;
            let min_cost = *vec![
                self.dtw_matrix[x - 1][y],
                self.dtw_matrix[x][y - 1],
                self.dtw_matrix[x - 1][y - 1],
            ]
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

            if self.dtw_matrix[x - 1][y] == min_cost {
                pos = (x - 1, y);
            } else if self.dtw_matrix[x][y - 1] == min_cost {
                pos = (x, y - 1);
            } else if self.dtw_matrix[x - 1][y - 1] == min_cost {
                pos = (x - 1, y - 1);
            }
            self.dtw_path.push(pos);
        }
        self.dtw_path.reverse();
        self.produce_aligned_sequence();
    }

    fn print_matrix<T: fmt::Display>(v: &Vec<Vec<T>>) {
        for i in 0..v.len() {
            for j in 0..v[0].len() {
                print!("{:1$.2$} ", v[i][j], 8, 2);
            }
            println!();
        }
    }

    pub fn produce_aligned_sequence(&mut self) {
        assert!(self.dtw_path.len() > 0);
        self.aligned_sequence = Vec::<Vector3<f64>>::with_capacity(self.dtw_path.len());

        for (x, y) in &self.dtw_path {
            self.aligned_sequence
                .push((self.seq1[*x - 1] + self.seq2[*y - 1]) / 2.0);
        }
    }

    pub fn consume_aligned_sequence(&mut self) -> Vec<Vector3<f64>> {
        /*
           Using this method will remove the aligned sequence from the struct
           To get another aligned sequence you must re-execute produce_aligned_sequence()
        */
        mem::take(&mut self.aligned_sequence)
    }
}

impl DTW for DTW3D {
    fn print_dtw_matrix(&self) {
        println!("Cost: {}", self.dtw_cost);
        let mut sliced_dtw_matrix =
            vec![vec![0.0; self.dtw_matrix[0].len() - 1]; self.dtw_matrix.len() - 1];

        for i in 1..self.dtw_matrix.len() {
            sliced_dtw_matrix[i - 1].copy_from_slice(&self.dtw_matrix[i][1..]);
        }

        Self::print_matrix(&sliced_dtw_matrix);
    }

    fn get_cost(&self) -> f64 {
        self.dtw_cost
    }

    fn print_dtw_path(&self) {
        let mut path_index: usize = 0;
        let width = 8;
        let precision = 2;
        println!("{:?}\n", self.seq1);
        println!("{:?}\n", self.seq2);
        println!("{:?}\n", self.dtw_path);
        println!("{:?}\n]", self.aligned_sequence);
        for i in 1..self.dtw_matrix.len() {
            for j in 1..self.dtw_matrix[i].len() {
                if (i, j) == self.dtw_path[path_index] {
                    print!(
                        "{}",
                        format!("{:1$.2$}", self.dtw_matrix[i][j], width, precision).on_green(),
                    );
                    path_index += 1;
                } else {
                    print!("{:1$.2$}", self.dtw_matrix[i][j], width, precision);
                }
            }
            println!();
        }
    }
}
