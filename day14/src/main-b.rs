use std::{collections::{BTreeMap, BTreeSet}, fs::read_to_string};

use regex::Regex;

fn main() {
    let config = (101, 103, "input.txt");

    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    
    let input: Vec<((i64, i64), (i64, i64))> = read_lines(config.2).into_iter().map(|line| {
        let (_, [px, py, vx, vy]) = re.captures(&line).unwrap().extract();
        let p: (i64, i64) = (px.parse().unwrap(), py.parse().unwrap());
        let v: (i64, i64) = (vx.parse().unwrap(), vy.parse().unwrap());
        (p, v)
    }).collect();

    // https://bsky.app/profile/josh.hawn.xyz/post/3ldaq6dwwts2a
    // In my case the puzzle is to solve A = 103x+70 = 101y+10.
    let mut step = 7383;
    let mut map: Vec<Vec<i32>> = Vec::new();
    for _ in 0..config.1 {
        map.push((0..config.0).map(|_| 0).collect());
    }
    for (p, v) in input.iter() {
        let pn = ((p.0 + step * v.0).rem_euclid(config.0), (p.1 + step * v.1).rem_euclid(config.1));
        map[pn.1 as usize][pn.0 as usize] += 1;
    }

    println!("{}", step);
    for row in map {
        for col in row {
            if col > 0 {print!("{}", col)}
            else {print!(".");}
        }
        println!("");
    }
    step += 1;
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}