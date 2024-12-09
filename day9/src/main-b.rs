use std::{collections::{BTreeMap, BTreeSet}, fs::read_to_string};

fn main() {
    
    let input: Vec<u8> = read_lines("input.txt")[0].as_bytes().iter().map(|u| u - b'0').collect();
    let mut files = Vec::new();
    let mut free_space = Vec::new();
    let mut p = 0;
    for i in 0..input.len() {
        match i % 2 {
            0 => files.push((p, input[i] as i64)),
            1 => free_space.push((p, input[i] as i64)),
            _ => panic!()
        }
        p += input[i] as i64;
    }

    let mut checksum = 0;
    for (i, (p, len)) in files.iter().enumerate().rev() {
        let move_to = free_space.iter_mut().filter(|(_, l)| *l >= *len).next();
        let p = match move_to {
            Some((p_fs, len_fs)) if *p_fs <= *p => {
                *len_fs -= *len;
                let r = *p_fs;
                *p_fs += *len;
                r
            }
            _ => *p
        };
        let d_checksum = (p + p + *len - 1) * (*len) / 2 * (i as i64);
        println!("{} {} {} {}", p, *len, i, d_checksum);
        checksum += d_checksum;
    }
    
    println!("{}", checksum);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}