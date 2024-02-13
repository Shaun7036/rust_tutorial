#![allow(unused)]

use std::io; 
use rand::Rng; 
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File; 
use std::cmp::Ordering;


// // TUTORIAL 1 : Std line stuff
// fn main() {
//     println!("What is your name?");
//     let mut name: String = String::new();
//     let greeting: &str = "Nice to meet you";
//     io::stdin().read_line(&mut name)
//         .expect("Didn't receive Input");

//     println!("Hello {}, {}", name.trim_end(), greeting);
// }

// // TUTORIAL 2: Constants & Shadowing
// fn main() {
//     const ONE_MIL: u32 = 1_000_000;
//     const PI: f32 = 3.141592;
//     let age: &str  = "47";// Important for strings - single quotes for chars and double quotes for strings;
//     // We can define variables with same name bu different data types (called shadowing)
//     // .expect is error handling built in while coding
//     let mut age: u32 = age.trim().parse()
//         .expect("Age wasn't assigned a number");
//     age = age + 1;
//     println!("I'm {} and I want ${}", age, ONE_MIL);

// }

// // TUTORIAL 3: Data Types
// fn main() {
//     // Integers (for signed integers, just use i32, i64 etc.)
//     // println!("Max u32: {}", u32::MAX);
//     // println!("Max u64: {}", u64::MAX);
//     // println!("Max u128: {}", u128::MAX);
//     // println!("Max usize: {}", usize::MAX);
//     // println!("Max f32: {}", f32::MAX);
//     // println!("Max f64: {}", f64::MAX);

//     // Booleans
//     // let is_true: bool = true;  // false
//     // // if we don't allow unused enabled;
//     // //we can start a variable with an underscore to make the compiler ignore it
//     // let my_grade: char = 'A';
// }

// TUTORIAL 4: Precision & Math
// fn main() {
    // let num_1: f32 = 1.111111111111;
    // println!("f32: {}", num_1 + 0.1111111111111111);
    // let num_2: f64 = 1.111111111111;
    // println!("f64: {}", num_2 + 0.1111111111111111);

    // let num_3: u32 = 5;
    // let num_4: u32 = 4;
    // let mut num_5: u32 = 5;
    // println!{"5 + 4 = {}", num_3 + num_4};
    // println!{"5 - 4 = {}", num_3 - num_4};
    // println!{"5 * 4 = {}", num_3 * num_4};
    // println!{"5 / 4 = {}", num_3 / num_4};
    // println!{"5 % 4 = {}", num_3 % num_4};
    // num_5 += 1;
    // println!("{}",num_5)

    // let random_num:


// Random Utility
// fn main() {
//     // cal the random thread to generate a random number from a range. 
//     let rand_num = rand::thread_rng().gen_range(1..101); 
//     println!("Random fucking number: {}", rand_num); 
// }


// // If Loops
// fn main() {
//     let age: i32 = 33; 
//     if ( age >= 1) && ( age <= 20){
//         println!("Young fuck {}", age); 
//     } else if ( age == 21) || ( age == 22) {
//         println!("Oldish fuck {}", age);
//     } else if (age >= 30){
//         println!("Aight cool")
//     } else {
//         println!("fucking die already"); 
//     }
// }

// // Ternary (shitty sub) 
// fn main() {
//     let mut age: i32 = 17; 
//     let can_vote: bool = if age >= 18 {
//         true
//     } else {
//         false
//     }; 
//     println!("Can you vote? {}", can_vote); 
// }

// Match: important that all cases are handled for instance all possibiities below. 
// Note the difference in syntax inside the match statement. 
// fn main() {
//     let age: i32 = 17; 
//     let voting_age: i32 = 18; 
//     // match age {
//     //     1..=18 => println!("You fuckiung suck"),
//     //     21 | 50 => println!("Eh"), 
//     //     65..=i32::MAX => println!("Old fuck"), 
//     //     _ => println!("You don't matter.")
//     // };

//     match age.cmp(&voting_age) {
//         Ordering::Less => println!("Can't Vote"),
//         Ordering::Greater => println!("Can vote"), 
//         Ordering::Equal => println!("huh"),
//     };
// }

// Arrays: Elements must be of the same type; and arrays have fixed size. 
// arr.len for array length (note rust compiler is smart enough to auto-size itself)
// looping through arrays: 
// fn main() {
    // let arr: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9]; 
    // println!("Arrays: {}", arr[0]); 
    // let mut loop_idx = 0; 
//     loop {
//         if arr[loop_idx] % 2 == 0 {
//             loop_idx += 1;
//             continue; 
//         }
//         if arr[loop_idx] == 9 {
//             break; 
//         }
//         println!("Values: {}", arr[loop_idx]);
//         loop_idx += 1;
//     }
// }

// // Other Loops: While & For Loop
// fn main() {
//     let arr: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9]; 
//     let mut loop_idx: usize = 0; 
//     // while loop_idx < arr.len() {
//     //     println!("{}", arr[loop_idx]);
//     //     loop_idx += 1;
//     // }
//     for val in arr.iter() {
//         println!("Val : {}", val); 
//     }

// }


// // Tuples
// fn main() {
//     let my_tuple: (i32, String, bool) = (43, "Bitch".to_string(), true);
//     if my_tuple.2 {
//         println!("{0} please get me {1} {0}es",my_tuple.1, my_tuple.0);
//     }
//     // can also assign variables values form a tuple
//     let (v1, v2, v3) = my_tuple;
//
// }

// Strings