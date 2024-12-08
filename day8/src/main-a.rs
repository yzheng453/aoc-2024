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
                let nx = i.0 * 2 - j.0;
                let ny = i.1 * 2 - j.1;
                if (nx >= 0) && (nx < n) && (ny >= 0) && (ny < m) {
                    antinodes.insert((nx, ny));
                }
            }
        }
    }
    
    println!("{}", antinodes.len());
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}