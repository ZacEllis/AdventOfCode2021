use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn most_common_bit(list: Vec<&str>, pos: usize) -> &str {
    let sum: u32 = list.iter().map(|l| l.chars().nth(pos).unwrap().to_digit(10).unwrap()).sum();
    if sum as f64 / list.len() as f64 >= 0.5 {
        "1"
    } else {
        "0"
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open(Path::new("input.txt"))?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let mut ox_vec: Vec<&str> = input.lines().collect();
    let mut co_vec = ox_vec.clone();

    for i in 0..ox_vec[0].len() {
        let common_bit = most_common_bit(ox_vec.clone(), i);
        ox_vec = ox_vec.iter().filter(|s| s.chars().nth(i).unwrap().to_string() == common_bit).cloned().collect();
        if ox_vec.len() == 1 {
            break;
        }
    }
    
    for i in 0..co_vec[0].len() {
        let common_bit = most_common_bit(co_vec.clone(), i);
        co_vec = co_vec.iter().filter(|s| s.chars().nth(i).unwrap().to_string() != common_bit).cloned().collect();
        if co_vec.len() == 1 {
            break;
        }
    }

    let ox_num = isize::from_str_radix(&ox_vec[0], 2).unwrap();
    let co_num = isize::from_str_radix(&co_vec[0], 2).unwrap();
    println!("{}", ox_num * co_num);

    Ok(())
}

// fn main() -> io::Result<()> {
//     let mut file = File::open(Path::new("input.txt"))?;
//     let mut input = String::new();
//     let mut num_lines = 0;
//     let mut digits = vec![0; 12];
//     let mut gamma = "".to_string();
//     let mut epsilon = "".to_string();
    
//     file.read_to_string(&mut input)?;
//     for line in input.lines() {
//         for i in 0..12 {
//             digits[i] += line.chars().nth(i).unwrap().to_digit(10).unwrap();
//         };
//         num_lines += 1;
//     };
//     for i in 0..12 {
//         if (digits[i] as f64 /num_lines as f64) > 0.5 {
//             gamma.push_str("1");
//             epsilon.push_str("0");
//         } else {
//             gamma.push_str("0");
//             epsilon.push_str("1");
//         }
//     };
//     let gamma_num = isize::from_str_radix(&gamma, 2).unwrap();
//     let eps_num = isize::from_str_radix(&epsilon, 2).unwrap();
//     println!("Result gamma {}, eps {}, {}", gamma_num, eps_num, gamma_num * eps_num);
//     Ok(())
// }