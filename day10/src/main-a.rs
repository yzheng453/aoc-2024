use std::{collections::{BTreeMap, BTreeSet}, fs::read_to_string};

static DIRECTIONS: &'static [(i64, i64)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];

fn main() {
    
    let map: Vec<Vec<u8>> = read_lines("input.txt").into_iter().map(|s| s.into_bytes()).collect();
    
    let mut sum = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == b'0' {
                sum += sch(&map, i, j);
            }
        }
    }
    
    println!("{}", sum);
}

fn sch(map: &Vec<Vec<u8>>, i: usize, j: usize) -> i64 {
    let mut ways = BTreeMap::new();
    //println!("({}, {})", i, j);
    ways.insert((i, j), 1 as i64);
    for id in b'1'..=b'9' {
        let last_ways = ways;
        ways = BTreeMap::new();
        for ((x, y), w) in last_ways {
            for (dx, dy) in DIRECTIONS {
                let nx = x as i64 + dx;
                let ny = y as i64 + dy;
                if let Ok(nx) = usize::try_from(nx) {
                    if let Ok(ny) = usize::try_from(ny) {
                        if let Some(h) = map.get(nx).and_then(|row| row.get(ny)) {
                            if *h == id {
                                match ways.get_mut(&(nx, ny)) {
                                    None => {
                                        ways.insert((nx, ny), w);
                                    },
                                    Some(c) => {
                                        *c += w;
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
        //println!("{} {}", id as char, ways.len());
    }
    ways.len() as i64
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}