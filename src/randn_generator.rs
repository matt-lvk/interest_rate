use ndarray::prelude::*;
// use ndarray::{Array, Array2};
// use rand::prelude::*;
use rand::distributions::{uniform::SampleUniform, Uniform};
use rand::Rng;
use rand_distr::{Distribution, Normal};
use num::Float;


/// Represents either a u32 or u64 number
#[derive(Debug)]
pub enum NumberType {
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
}


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
pub fn gaussian_generator_array<T>(mean: T, std_dev: T, size: usize) 
-> Array1<T>
where
    T: Float + SampleUniform
    {
        let normal = Normal::new(mean.to_f64().unwrap(),
                                    std_dev.to_f64().unwrap()).unwrap();
        let mut rng = rand::thread_rng();
        let mut array = Array1::<T>::zeros(size);
        for i in 0..size {
            let v: f64 = normal.sample(&mut rng);
            array[i] = T::from(v).unwrap();
        }
        return array;
    }