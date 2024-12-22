use std::{collections::{BTreeMap, BTreeSet, VecDeque}, fs::read_to_string, iter, ops::Div};

use regex::Regex;

fn main() {
    let input: Vec<String> = read_lines("input.txt");
    let sum: i64 = input.into_iter().map(|s| {
        let mut sn: i64 = s.parse().unwrap();
        for i in 0..2000 {
            sn = mix_and_prune(sn * 64, sn);
            sn = mix_and_prune(sn >> 5, sn);
            sn = mix_and_prune(sn * 2048, sn);
        }
        println!("{}", sn);
        sn
    }).sum();
    println!("{}", sum);
}

fn mix_and_prune(a: i64, b: i64) -> i64 {
    (a ^ b) % 16777216
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}