use colored::Colorize;
use std::fmt;
#[derive(Default)]
pub struct DTW1D {
    seq1: Vec<f64>,
    seq2: Vec<f64>,
    cost_matrix: Vec<Vec<f64>>,
    dtw_matrix: Vec<Vec<f64>>,
    dtw_path: Vec<(usize, usize)>,
    dtw_cost: f64,
    normalized_dtw_cost: f64,
}

pub trait DTW {
    fn print_dtw_matrix(&self);

    fn get_cost(&self) -> f64;

    fn print_dtw_path(&self);
}

impl DTW1D {
    pub fn new(seq1: Vec<f64>, seq2: Vec<f64>) -> DTW1D {
        let mut dtw = DTW1D {
            seq1,
            seq2,
            ..Default::default()
        };

        dtw.create_cost_matrix();
        dtw.create_dtw_matrix();
        dtw.create_dtw_path();

        dtw
    }

    fn create_cost_matrix(&mut self) {
        self.cost_matrix = vec![vec![Default::default(); self.seq2.len()]; self.seq1.len()];

        for i in 0..self.cost_matrix.len() {
            for j in 0..self.cost_matrix[0].len() {
                self.cost_matrix[i][j] = (self.seq1[i] - self.seq2[j]).abs()
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
    }

    fn print_matrix<T: fmt::Display>(v: &Vec<Vec<T>>) {
        for i in 0..v.len() {
            for j in 0..v[0].len() {
                print!("{:1$.2$} ", v[i][j], 8, 2);
            }
            println!();
        }
    }
}

impl DTW for DTW1D {
    fn get_cost(&self) -> f64 {
        self.dtw_cost
    }
    fn print_dtw_matrix(&self) {
        let mut sliced_dtw_matrix =
            vec![vec![0.0; self.dtw_matrix[0].len() - 1]; self.dtw_matrix.len() - 1];

        for i in 1..self.dtw_matrix.len() {
            sliced_dtw_matrix[i - 1].copy_from_slice(&self.dtw_matrix[i][1..]);
        }

        Self::print_matrix(&sliced_dtw_matrix);
    }

    fn print_dtw_path(&self) {
        let mut path_index: usize = 0;
        let width = 8;
        let precision = 2;

        print!("{:<1$}", " ", width);
        for i in &self.seq2 {
            print!("{:1$.2$}", *i, width, precision);
        }

        println!();

        for i in 1..self.dtw_matrix.len() {
            print!("{:1$.2$}", self.seq1[i - 1], width, precision);
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
