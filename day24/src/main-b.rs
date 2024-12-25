use std::{collections::{BTreeMap, BTreeSet, VecDeque}, fs::read_to_string, ops::RangeBounds};

use regex::Regex;
use std::ops::Bound::{Included, Unbounded};
use rand::{Rng};

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
    let mut output_wires = BTreeSet::new();
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
            output_wires.insert(output);
            let op = if op.starts_with('X') { Gate::XOR }
            else if op.starts_with('A') { Gate::AND }
            else { Gate::OR };
            gates.entry(left).or_insert_with(|| Vec::new())
                .push((right, op, output));
            
            gates.entry(right).or_insert_with(|| Vec::new())
                .push((left, op, output));
        }
    }
    let mut swap = BTreeMap::new();
    let mut rng = rand::thread_rng();
    
    let mut scores = Vec::new();
    let mut count = 0;
    for w0 in output_wires.iter() {
        if swap.contains_key(w0) {
            continue;
        }
        for w1 in output_wires.range(*w0..) {
            if swap.contains_key(w1) {
                continue;
            }
            count += 1;
            if count % 1000 == 0 {
                println!("{}", count);
            }
            swap.insert(*w0, *w1);
            swap.insert(*w1, *w0);
            
            let mut sum = 0;
            for _ in 0..500 {
                let x = rng.gen_range(0..(1<<45));
                let y = rng.gen_range(0..(1<<45));
                sum += test_xy(x, y, &swap, &state, &gates);
            }
            
            scores.push((*w0, *w1, sum));
            
            swap.remove(w0);
            swap.remove(w1);
        }
    }
    
    scores.sort_by(|x, y| x.2.cmp(&y.2));
    
    for s in &scores[0..4] {
        swap.insert(s.0, s.1);
        swap.insert(s.1, s.0);
        
    }

    let mut sum = 0;
    for _ in 0..500 {
        let x = rng.gen_range(0..(1<<45));
        let y = rng.gen_range(0..(1<<45));
        sum += test_xy(x, y, &swap, &state, &gates);
    }
    println!("{} {:?}", sum, swap.keys());
}

fn test_xy(x_in: i64, y_in: i64, swap: &BTreeMap<[u8; 3], [u8; 3]>, r_state: &BTreeMap<[u8; 3], i32>, gates: &BTreeMap<[u8; 3], Vec<([u8; 3], Gate, [u8; 3])>>) -> i64 {
    let mut state = r_state.clone();
    let (mut x, mut y) = (x_in, y_in);
    for (_, v) in state.range_mut((Included([b'x', b'0', b'0']), Included([b'x', b'9', b'9']))) {
        *v = (x % 2) as i32;
        x /= 2;
    }
        
    for (_, v) in state.range_mut((Included([b'y', b'0', b'0']), Included([b'y', b'9', b'9']))) {
        *v = (y % 2) as i32;
        y /= 2;        
    }

    let mut z= evaluate(&mut state, gates, swap);
    let mut z_true = x_in + y_in;
    let mut count = 0;
    while z > 0 || z_true > 0 {
        count += (z%2) ^ (z_true%2);
        z = z / 2;
        z_true = z_true / 2;
    }
    return count
} 

fn evaluate(state: &mut BTreeMap<Wire, i32>, gates: &BTreeMap<[u8; 3], Vec<([u8; 3], Gate, [u8; 3])>>, swap: &BTreeMap<Wire, Wire>) -> i64 {
    let mut queue: VecDeque<Wire> = state.keys().cloned().collect();
    while let Some(w) = queue.pop_front() {
        let sl = state[&w];
        for (right, op, output) in gates.get(&w).unwrap_or(&Vec::new()) {
            let output = swap.get(output).unwrap_or(output);
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
    let result = state.range((Included([b'z', b'0', b'0']), Unbounded)).rev().fold(0 as i64, |acc, x| acc * 2 + (*x.1 as i64));
    result
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
