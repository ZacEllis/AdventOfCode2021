use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() -> io::Result<()> {
    let mut file = File::open(Path::new("input.txt"))?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let mut increases = 0;
    let depths: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut prev = -1;
    for i in 0..(depths.len()-2) {
        let sum = depths[i] + depths[i+1] + depths[i+2];
        if prev != -1 {
            if sum > prev {
                increases += 1;
            }
            
        }
        prev = sum;
    }
    println!("Result: {}", increases);
    Ok(())
}

// Part 1
// fn main() -> io::Result<()> {
//     let mut file = File::open(Path::new("input.txt"))?;
//     let mut input = String::new();
//     file.read_to_string(&mut input)?;
//     let mut prev = -1;
//     let mut increases = 0;
//     for line in input.lines() {
//         curr = line.parse::<i32>().unwrap();
//         if prev != -1 {
//             if curr > prev {
//                 increases += 1;
//             }
//         }
//         prev = curr;
//     }
//     println!("Result: {}", increases);
//     Ok(())
// }