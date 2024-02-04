#![allow(unused)]

use std::io; 
use rand::Rng; 
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File; 
use std::cmp::Ordering; 

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

// }

// Strings