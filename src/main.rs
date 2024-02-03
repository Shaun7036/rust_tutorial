#![allow(unused)]

use std::io; 
use rand::Rng; 
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File; 
use std::cmp::Ordering; 

// Random
fn main() {
    // cal the random thread to generate a random number from a range. 
    let rand_num = rand::thread_rng().gen_range(1..101); 
    println!("Random fucking number: {}", rand_num); 

}

