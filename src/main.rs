/**
 * Homework Number Generator
 * Dec 4, 2022
 * Edward Naidoo
 */

use rand::Rng;
use round::round_up;

fn main() {
    println!();
    generate_iy();
    generate_pv();
    generate_fv();
    generate_pmt();
    generate_years();
    println!();
}
      

fn generate_iy() {
    // Generate random number with "%"
    let percent = rand::thread_rng().gen_range(1.0..16.0);
    
    // Rounds float to two decimal places
    println!("I/Y = {}%", round_up(percent, 2));
}

fn generate_pv() {
    // Generate random number with "$"
    let pv = rand::thread_rng().gen_range(1.0..10000.0);
    
    // Rounds float to two decimal places
    println!("PV = ${}", round_up(pv, 2));
}

fn generate_fv() {
    // Generate random number with "$"
    let fv = rand::thread_rng().gen_range(1.0..10000.0);
    
    // Rounds float to two decimal places
    println!("FV = ${}", round_up(fv, 2));
}

fn generate_pmt() {
    // Generate random number with "$"
    let pmt = rand::thread_rng().gen_range(1.0..1000.0);
    
    // Rounds float to two decimal places
    println!("PMT = ${}", round_up(pmt, 2));
}

fn generate_years() {
    // Generate random number with "_years"
    let years = rand::thread_rng().gen_range(1..36);
    
    // Rounds float to two decimal places
    if years > 1 {
        println!("Time = {} years", years);
    } else {
        println!("Time = {} year", years);
    }

}