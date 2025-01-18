// use ndarray::prelude::*;
// use ndarray::{Array, Array2};
// use ndarray_csv::{Array2Reader, Array2Writer};
// use rand_distr::{Normal, Distribution};
// use rand::prelude::*;
// use csv::{ReaderBuilder, WriterBuilder};
// use std::fs::File;
// use std::error::Error;
// use std::io;
// use ndarray_npy::write_npy;

// struct HullWhiteParameters {
//     a: f64,
//     sigma: f64,
//     forward_rates: ndarray::Array1<f64>,
//     start_t: f64,
//     T: f64,
//     nsim: i32,
//     dt: f64,
// }

// #[derive(Clone)]
// struct Vasicek {
//     a: f64,
//     sigma: f64,
//     theta: f64,
//     start_t: f64,
//     T: f64,
//     nsim: i32,
//     dt: f64,
// }

// fn wiener_random_walk(nsim: i32, T: usize, df: f64) -> Array2<f64> {
//     let mut rng = rand::thread_rng();
//     let mut w: Array2<f64> = Array::zeros((nsim as usize, T));
//     for i in 0..nsim {
//         for j in 1..T {
//             w[[i as usize, j]] = w[[i as usize, j - 1]] 
//                 + gaussian_number_generator(0.0, 1.0) * df.sqrt();
//         }
//     }
//     return w;
// }

// fn short_rate_vasicek(varsicek: Vasicek, wiener_mat: Array2<f64>) -> Array2<f64> {
//     let mut r: Array2<f64> = Array::zeros((varsicek.nsim as usize, wiener_mat.shape()[1]));
//     for i in 0..varsicek.nsim {
//         r[[i as usize, 0]] = varsicek.start_t;
//         for j in 1..wiener_mat.shape()[1] {
//             r[[i as usize, j]] = r[[i as usize, j - 1]] 
//                 + varsicek.a * (varsicek.theta - r[[i as usize, j - 1]]) * varsicek.dt
//                 + varsicek.sigma * wiener_mat[[i as usize, j]] * varsicek.dt.sqrt();
//         }
//     }
//     return r;
// }

// fn main() {
//     println!("Start");

//     let varsicek: Vasicek = Vasicek {
//         a: 0.1,
//         sigma: 0.01,
//         theta: 0.5,
//         start_t: 0.0,
//         T: 500.0,
//         nsim: 500,
//         dt: 1.0,
//     };

//     let t: Array1<f64> = Array::range(0.0,
//                                         varsicek.T, 
//                                         varsicek.dt);

//     let wiener_mat: Array2<f64> = wiener_random_walk(
//                                     varsicek.nsim, 
//                                     varsicek.T as usize,
//                                     varsicek.dt
//                                 );
//     let short_rate: Array2<f64> = short_rate_vasicek(varsicek.clone(), wiener_mat);

//     // Write to NPY
//     write_npy("array.npy", &short_rate);
    
//     println!("End");
// }
