use std::{collections::{BTreeMap, BTreeSet, VecDeque}, fs::read_to_string, iter, ops::Div};

use regex::Regex;

fn main() {
    let input: Vec<String> = read_lines("input.txt");
    let mut m: BTreeMap<(i32, i32, i32, i32), (usize, i32)> = BTreeMap::new();
    for (i, s) in input.into_iter().enumerate() {
        let mut sn: i64 = s.parse().unwrap();
        let mut prices: Vec<i32> = vec![(sn%10) as i32; 1];
        for j in 0..2000 {
            sn = mix_and_prune(sn * 64, sn);
            sn = mix_and_prune(sn >> 5, sn);
            sn = mix_and_prune(sn * 2048, sn);
            prices.push((sn % 10) as i32);
        }
        for j in 1..(2000 - 4) {
            let r = (prices[j] - prices[j-1], prices[j+1] - prices[j], prices[j+2] - prices[j+1], prices[j+3] - prices[j+2]);
            //println!("{} {:?} {}", i, r, prices[j+3]);
            m.entry(r)
                .and_modify(|e| {
                    if e.0 < i {
                        *e = (i, e.1 + prices[j+3]);
                    }
                })
                .or_insert((i, prices[j + 3]));
        }
    }
    let max = m.values().map(|e| e.1).max().unwrap();
    println!("{}", max);
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