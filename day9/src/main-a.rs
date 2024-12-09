use std::{collections::{BTreeMap, BTreeSet}, fs::read_to_string};

fn main() {
    
    let input: Vec<u8> = read_lines("input.txt")[0].as_bytes().iter().map(|u| u - b'0').collect();
    let mut files = Vec::new();
    for i in (0..input.len()).step_by(2) {
        files.push(input[i] as i64);
    }

    let mut p = 0;
    let mut checksum = 0;
    for (i, &v) in input.iter().enumerate() {
        let mut v = v as i64;
        match i % 2 {
            0 => {
                // File block
                let file_id = i / 2;
                v = files[file_id];
                println!("{} {} {} {}", p, v, file_id, checksum);
                checksum += (p + p + v - 1) * v / 2 * file_id as i64;
                p += v;
                
                if file_id == files.len() - 1 {
                    break;
                }
            }
            1 => {
                // Free space
                while v > 0 {
                    let file_id = files.len() - 1;
                    let file_size = files[file_id];
                    let fwd_step = if file_size <= v {
                        files.pop();
                        file_size
                    } else {
                        files[file_id] -= v;
                        v
                    };
                    println!("{} {} {} {}", p, fwd_step, file_id, checksum);
                    v -= fwd_step;
                    checksum += (p + p + fwd_step - 1) * fwd_step / 2 * (file_id as i64);
                    p += fwd_step;
                }
            }
            _ => panic!()
        }
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