use std::{collections::{BTreeMap, BTreeSet, VecDeque}, fs::read_to_string, ops::RangeBounds};

use regex::Regex;

type Schema = [i32];

fn main() {
    let mut it = read_lines("input.txt").into_iter();
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    while let Some(line) = it.next() {
        let mut current = vec![line.into_bytes(); 1];
        while let Some(line) = it.next() {
            if line.len() == 0 {
                break;
            }
            current.push(line.into_bytes());
        }
        let mut s = vec![0; current[0].len()];
        for i in 0..current.len() - 1 {
            for j in 0..current[i].len() {
                if current[i][j] != current[i+1][j] {
                    s[j] = i;
                }
            }
        }
        if current[0][0] == b'#' {
            locks.push(s);
        } else {
            keys.push(s);
        }
    }
    
    let mut sum = 0;
    for lock in locks {
        for key in keys.iter() {
            if lock.iter().zip(key.iter()).all(|(l, k)| *l <= *k) {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
    
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
