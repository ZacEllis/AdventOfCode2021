use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() -> io::Result<()> {
    let mut file = File::open(Path::new("input.txt"))?;
    let mut input = String::new();
    let mut side = 0;
    let mut depth = 0;
    let mut aim = 0;
    file.read_to_string(&mut input)?;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let command = parts[0];
        let dist = parts[1].parse::<i32>().unwrap();
        match command {
            "forward" => {
                side += dist;
                depth += aim * dist;
            },
            "up" => aim -= dist,
            "down" => aim += dist, 
            _ => println!("wtf")
        };
    }
    
    println!("Result: depth {}, side {}, mul {}", depth, side, depth * side);
    Ok(())
}

// fn main() -> io::Result<()> {
//     let mut file = File::open(Path::new("input.txt"))?;
//     let mut input = String::new();
//     let mut side = 0;
//     let mut depth = 0;
//     file.read_to_string(&mut input)?;
//     for line in input.lines() {
//         let parts: Vec<&str> = line.split(" ").collect();
//         let command = parts[0];
//         let dist = parts[1].parse::<i32>().unwrap();
//         match command {
//             "forward" => side += dist,
//             "up" => depth -= dist,
//             "down" => depth += dist, 
//             _ => println!("wtf")
//         };
//     }
    
//     println!("Result: depth {}, side {}, mul {}", up, side, up * side);
//     Ok(())
// }

