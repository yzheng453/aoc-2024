use std::{collections::{BTreeMap, BTreeSet}, fs::read_to_string};

static DIRECTIONS: &'static [(i64, i64)] = &[(-1, 0), (1, 0), (0, 1), (0, -1)];

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
    for (dx, dy) in DIRECTIONS {
        let (da, dp) = try_direction(x, dx, y, dy, map, u, visited);
        a += da;
        p += dp;
    }
    (a, p)
}

fn try_direction(x: usize, dx: &i64, y: usize, dy: &i64, map: &Vec<Vec<u8>>, u: u8, visited: &mut Vec<Vec<bool>>) -> (i64, i64) {
    if let Ok(nx) = usize::try_from(x as i64 + dx) {
        if let Ok(ny) = usize::try_from(y as i64 + dy) {
            if map.get(nx).and_then(|r| r.get(ny)).is_some_and(|v| *v == u) {
                if !visited[nx][ny] {
                    return sch(map, visited, nx, ny, u);
                } else {
                    return (0, 0);
                }
            }
        }
    }
    (0, 1)
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}