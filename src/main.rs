mod randn_generator;

use randn_generator::{gaussian_generator, 
                        gaussian_generator_array, 
                        NumberType};
use rand::distributions::uniform::SampleUniform;
use rand_distr::{Distribution, Normal};
use rand::{distributions::Uniform, Rng};

fn main() {
    println!("Start");
    let n: f64 = gaussian_generator(2.0, 5.0);
    let r_array = gaussian_generator_array(
                                    2.0, 5.0, 10000000);
    println!("random normal number is: {0}", n);
    println!("random normal array is: {0}", r_array);
}