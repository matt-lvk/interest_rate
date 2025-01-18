use ndarray::prelude::*;
use ndarray::{Array, Array2};
use rand::prelude::*;
use rand_distr::{Distribution, Normal};

fn gaussian_number_generator(mean: f64, std_dev: f64) -> f64 {
    let normal: Normal<f64> = Normal::new(mean, std_dev).unwrap();
    let v: f64 = normal.sample(&mut rand::thread_rng());
    return v;
}