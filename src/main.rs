/**
 * Homework Number Generator
 * Dec 4, 2022
 * Edward Naidoo
 */

use rand::Rng;
use round::round_up;

fn main() {
    generate_percent();
    generate_amount();
}
      

fn generate_percent() {
    // Generate random number with "%"
    let percent = rand::thread_rng().gen_range(1.0..16.0);
    
    // Rounds float to two decimal places
    println!("{}%", round_up(percent, 2));
}

fn generate_amount() {
    // Generate random number with "%"
    let percent = rand::thread_rng().gen_range(1.0..10000.0);
    
    // Rounds float to two decimal places
    println!("${}", round_up(percent, 2));
}