use ndarray::prelude::*;
use ndarray::{Array, Array2};
use rand::prelude::*;
use rand::distributions::uniform::SampleUniform;
use rand_distr::{Distribution, Normal};
use num::Float;

/// Generates a random number from a Gaussian distribution with the given mean and standard deviation.
///
/// # Arguments
///
/// * `mean` - The mean of the Gaussian distribution.
/// * `std_dev` - The standard deviation of the Gaussian distribution.
///
/// # Returns
///
/// A random number sampled from the specified Gaussian distribution.
pub fn gaussian_generator<T>(mean: T, std_dev: T) -> T 
where
    T: Float + SampleUniform,
{
    let normal = Normal::new(mean.to_f64().unwrap(),
                            std_dev.to_f64().unwrap()).unwrap();
    let v: f64 = normal.sample(&mut rand::thread_rng());
    return T::from(v).unwrap();
}

fn gaussian_generator_array(mean: f64, std_dev: f64) -> f64 {
    let normal: Normal<f64> = Normal::new(mean, std_dev).unwrap();
    let v: f64 = normal.sample(&mut rand::thread_rng());
    return v;
}