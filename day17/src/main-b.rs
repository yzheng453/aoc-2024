use std::{collections::BTreeSet, fs::read_to_string, thread::panicking};

fn main() {

    let mut iter= read_lines("input.txt").into_iter();
    let _orig_a: i64 = iter.next().unwrap().split(": ").skip(1).next().unwrap().parse().unwrap();
    let init_b: i64 = iter.next().unwrap().split(": ").skip(1).next().unwrap().parse().unwrap();
    let init_c: i64 = iter.next().unwrap().split(": ").skip(1).next().unwrap().parse().unwrap();
    iter.next();
    let program: Vec<i64> = iter.next().unwrap().split(": ").skip(1).next().unwrap().split(',').map(|n| n.parse::<i64>().unwrap()).collect();
    
    let mut init_a_store = vec![vec![0; 1]; 1];
    for instruct in program.iter().rev() {
        let mut current_candidates = Vec::new();
        for &init_a in init_a_store.last().unwrap() {
            for last_a in 0..8 {
                let mut a = init_a * 8 + last_a;
                let mut b = init_b;
                let mut c = init_c;
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
                        5 => {
                            let v = combo_op % 8;
                            if v == *instruct {
                                current_candidates.push(init_a * 8 + last_a);
                            }
                            break;
                        },
                        6 => b = a >> combo_op,
                        7 => c = a >> combo_op,
                        _ => panic!()
                    };
                    ic += 2;
                }
            }
        }
        init_a_store.push(current_candidates);
    }
    println!("{}", init_a_store.last().unwrap().iter().min().unwrap());
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}