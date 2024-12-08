use std::{collections::{BTreeMap, BTreeSet}, fs::read_to_string};

fn main() {
    
    let map: Vec<Vec<u8>> = read_lines("input.txt").into_iter().map(|line| line.into_bytes()).collect();
    let n = map.len();
    let m = map[0].len();
    let mut nodes: BTreeMap<u8, Vec<(i64, i64)>> = BTreeMap::new();
    for i in 0..n {
        for j in 0..m {
            let freq = map[i][j];
            if map[i][j] != b'.' {
                let p = (i as i64, j as i64);
                match nodes.get_mut(&freq) {
                    Some(v) => v.push(p),
                    None => {
                        nodes.insert(freq, vec![p; 1]);
                    }
                }
            }
        }
    }
    
    let n = n as i64;
    let m = m as i64;
    
    let mut antinodes = BTreeSet::new();
    for nodes_of_f in nodes.values() {
        for i in nodes_of_f {
            for j in nodes_of_f {
                if i == j {
                    continue;
                }
                let (dx, dy) = calc_dist(i.0 - j.0, i.1 - j.1);
                let mut nx = i.0;
                let mut ny = i.1;
                while (nx >= 0) && (nx < n) && (ny >= 0) && (ny < m) {
                    antinodes.insert((nx, ny));
                    nx += dx;
                    ny += dy;
                }
            }
        }
    }
    
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let a = i as i64;
            let b = j as i64;
            if antinodes.contains(&(a, b)) {
                print!("#");
            } else {
                print!("{}", map[i][j] as char)
            }
        }
        println!("")
    }
    
    println!("{}", antinodes.len());
}

fn calc_dist(dx: i64, dy: i64) -> (i64, i64) {
    let g = gcd(dx.abs(), dy.abs());
    (dx / g, dy / g)
}

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (b, a % b);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}