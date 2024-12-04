use std::fs::read_to_string;
use regex::Regex;

const WORD: &'static [u8] = &[b'X', b'M', b'A', b'S'];
const DIRECTIONS: &'static [(i64, i64)] = &[(0, 1), (0, -1), (1, 0), (-1, 0), (1,1), (-1, -1), (1, -1), (-1, 1)];

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn search_word(p: &Vec<Vec<u8>>, dx: i64, dy: i64) -> i64 {
    let n = p.len();
    let m = p[0].len();    
    let mut s = 0;
    for i in 0..n {
        for j in 0..m {
            let mut f = true;
            for k in 0..WORD.len() {
                let mut f1 = false;
                let x = i as i64 + dx * k as i64;
                let y = j as i64 + dy * k as i64;
                if let Ok(x1) = usize::try_from(x) {
                    if let Ok(y1) = usize::try_from(y) {
                        if p.get(x1).and_then(|row| row.get(y1)).is_some_and(|&u| u == WORD[k]) {
                            f1 = true;
                        } 
                    }
                }
                if !f1 {
                    f = false;
                    break;
                }
            }
            if f {
                s += 1;
            }
        }
    }
    s
}

fn main() {
    
    let p: Vec<Vec<u8>> = read_lines("input.txt").into_iter()
        .map(|s| s.into_bytes()).collect();
    
    let sum: i64 = DIRECTIONS.iter().map(|(dx, dy)| search_word(&p, *dx, *dy)).sum();

    println!("{}", sum);

}

