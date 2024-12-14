use std::{collections::{BTreeMap, BTreeSet}, fs::read_to_string};

use regex::Regex;

fn main() {
    //let config = (11, 7, "test_input.txt", 100);
    let config = (101, 103, "input.txt", 100);

    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    
    let mut c = vec![vec![0; 2]; 2];

    for line in read_lines(config.2) {
        let (_, [px, py, vx, vy]) = re.captures(&line).unwrap().extract();
        let p: (i64, i64) = (px.parse().unwrap(), py.parse().unwrap());
        let v: (i64, i64) = (vx.parse().unwrap(), vy.parse().unwrap());
        let pn = ((p.0 + config.3 * v.0).rem_euclid(config.0), (p.1 + config.3 * v.1).rem_euclid(config.1));
        println!("{} {}", pn.0, pn.1);
        let c0 = if pn.0 < config.0/2 { 0 } else if pn.0 > config.0/2 {1} else {continue;};
        let c1 = if pn.1 < config.1/2 { 0 } else if pn.1 > config.1/2 {1} else {continue;};
        c[c0 as usize][c1 as usize] += 1;
    }
    
    println!("{}", c[0][0] * c[0][1] * c[1][0] * c[1][1]);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}