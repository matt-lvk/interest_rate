mod randn_generator;

use randn_generator::gaussian_generator;


fn main() {
    println!("Start");
    let n: f64 = gaussian_generator(2.0, 5.0);
    println!("random number is: {0}", n);
    println!("End");
}