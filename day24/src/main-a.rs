use std::{collections::{BTreeMap, BTreeSet, VecDeque}, fs::read_to_string, ops::RangeBounds};

use regex::Regex;
use std::ops::Bound::{Included, Unbounded};


#[derive(Clone, Copy)]
enum Gate {
    XOR,
    AND,
    OR,
}

type Wire = [u8; 3];

fn main() {
    let mut state: BTreeMap<Wire, i32> = BTreeMap::new();
    let mut gates: BTreeMap<Wire, Vec<(Wire, Gate, Wire)>> = BTreeMap::new();
    let mut input_mode = 0;
    let re0 = Regex::new(r"(\w{3}): (\d)").unwrap();
    let re1 = Regex::new(r"(\w{3}) (\w{2,3}) (\w{3}) -> (\w{3})").unwrap();

    for line in read_lines("input.txt") {
        if input_mode == 0 {
            if line.len() == 0 {
                input_mode = 1;
                continue;
            }
            let (_, [wire, value]) = re0.captures(&line).unwrap().extract();
            let wire: Wire = wire.as_bytes().try_into().unwrap();
            state.insert(wire, if value.starts_with('1') { 1 } else { 0 });
        } else {
            let (_, [left, op, right, output]) = re1.captures(&line).unwrap().extract();
            let left: Wire = left.as_bytes().try_into().unwrap();
            let right: Wire = right.as_bytes().try_into().unwrap();
            let output: Wire = output.as_bytes().try_into().unwrap();
            let op = if op.starts_with('X') { Gate::XOR }
            else if op.starts_with('A') { Gate::AND }
            else { Gate::OR };
            gates.entry(left).or_insert_with(|| Vec::new())
                .push((right, op, output));
            
            gates.entry(right).or_insert_with(|| Vec::new())
                .push((left, op, output));
        }
    }
    let mut queue: VecDeque<Wire> = state.keys().cloned().collect();
    while let Some(w) = queue.pop_front() {
        let sl = state[&w];
        for (right, op, output) in gates.get(&w).unwrap_or(&Vec::new()) {
            if let Some(sr) = state.get(right) {
                if !state.contains_key(output) {
                    let output_v = match *op {
                        Gate::AND => sl & sr,
                        Gate::OR => sl | sr,
                        Gate::XOR => sl ^ sr,
                    };
                    state.insert(*output, output_v);
                    queue.push_back(*output);
                }
            }
        }
    }
    
    for i in state.range((Included([b'z', b'0', b'0']), Unbounded)) {
        println!("{:?}", i);
    }
    let result = state.range((Included([b'z', b'0', b'0']), Unbounded)).rev().fold(0 as i64, |acc, x| acc * 2 + (*x.1 as i64));
    println!("{}", result);
    
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
