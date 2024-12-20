use std::{collections::{BTreeMap, BTreeSet, VecDeque}, fs::read_to_string};

fn main() {
    let mut input = read_lines("input.txt").into_iter();
    let towels_s =  input.next().unwrap();
    let towels: Vec<_> = towels_s.split(", ").collect();
    input.next();
    let designs: Vec<_> = input.collect();
    
    let sum: i32 = designs.iter().map(|design| {
        let n = design.len();
        let mut f = vec![false; n + 1];
        f[0] = true;
        for i in 0..=n {
            if f[i] {
                for &towel in towels.iter() {
                    if design[i..].starts_with(towel) {
                        f[i + towel.len()] = true;
                    }
                }
            }
        }
        if f[n] {
            1
        } else {
            0
        }
    }).sum();

    println!("{}", sum);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}