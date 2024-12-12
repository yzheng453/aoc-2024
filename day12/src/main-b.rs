use std::{collections::{BTreeMap, BTreeSet}, convert::identity, fs::read_to_string};

static DIRECTIONS: &'static [(i64, i64)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    
    let map: Vec<Vec<u8>> = read_lines("input.txt").into_iter().map(|s| {
        s.into_bytes()
    }).collect();
    
    let mut sum = 0;
    let mut visited: Vec<Vec<bool>> = (0..map.len()).map(|_| vec![false; map[0].len()]).collect();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if visited[i][j] {
                continue;
            }
            let (a, p) = sch(&map, &mut visited, i, j, map[i][j]);
            println!("{} {} {}", map[i][j] as char, a, p);
            sum += a * p;
        }
    }
    
    println!("{}", sum);
}


fn sch(map: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>, x: usize, y: usize, u: u8) -> (i64, i64) {
    visited[x][y] = true;
    let mut a = 1;
    let mut p = 0;
    let mut shape = Vec::new();
    for (dx, dy) in DIRECTIONS {
        let (da, dp, identical) = try_direction(x, *dx, y, *dy, map, u, visited);
        a += da;
        p += dp;
        shape.push(identical);
    }
    for i in 0..4 {
        let ni = (i+1) % 4;
        if shape[i] == shape[ni] {
            if shape[i] {
                let (dx, dy) = (DIRECTIONS[i].0 + DIRECTIONS[ni].0, DIRECTIONS[i].1 + DIRECTIONS[ni].1);
                if compare(x, dx, y, dy, map, u).is_none() {
                    p += 1;
                }
            } else {
                p += 1;
            }
        }
    }
    (a, p)
}

fn try_direction(x: usize, dx: i64, y: usize, dy: i64, map: &Vec<Vec<u8>>, u: u8, visited: &mut Vec<Vec<bool>>) -> (i64, i64, bool) {
    if let Some((nx, ny)) = compare(x, dx, y, dy, map, u) {
        if !visited[nx][ny] {
            let c = sch(map, visited, nx, ny, u);
            return (c.0, c.1, true);
        } else {
            return (0, 0, true);
        }
    }
    (0, 0, false)
}

fn compare(x: usize, dx: i64, y: usize, dy: i64, map: &Vec<Vec<u8>>, u: u8) -> Option<(usize, usize)> {
    if let Ok(nx) = usize::try_from(x as i64 + dx) {
        if let Ok(ny) = usize::try_from(y as i64 + dy) {
            if map.get(nx).and_then(|r| r.get(ny)).is_some_and(|v| *v == u) {
                return Some((nx, ny));
            }
        }
    }
    None
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}