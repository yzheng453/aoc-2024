use std::{collections::{BTreeMap, BTreeSet}, fs::read_to_string};
use std::ops::Bound::Included;

static DIRECTIONS: &'static [(i64, i64)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {

    let mut iter= read_lines("input.txt").into_iter();
    let mut a: i64 = iter.next().unwrap().split(": ").skip(1).next().unwrap().parse().unwrap();
    let mut b: i64 = iter.next().unwrap().split(": ").skip(1).next().unwrap().parse().unwrap();
    let mut c: i64 = iter.next().unwrap().split(": ").skip(1).next().unwrap().parse().unwrap();
    iter.next();
    let program: Vec<i64> = iter.next().unwrap().split(": ").skip(1).next().unwrap().split(',').map(|n| n.parse::<i64>().unwrap()).collect();
    let mut ic = 0;
    while ic < program.len() {
        let literal_op= program[ic + 1];
        let combo_op = match literal_op {
            0..=3 => literal_op,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!()
        };
        match program[ic] {
            0 => a = a >> combo_op,
            1 => b = b ^ literal_op,
            2 => b = combo_op % 8,
            3 => if a != 0 { ic = literal_op as usize; continue; },
            4 => b = b ^ c,
            5 => print!("{},", combo_op % 8),
            6 => b = a >> combo_op,
            7 => c = a >> combo_op,
            _ => panic!()
        };
        ic += 2;
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}