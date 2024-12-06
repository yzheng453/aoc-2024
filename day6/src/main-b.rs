use std::{collections::{BTreeMap, BTreeSet}, convert::identity, fs::read_to_string};

const DIRECTIONS: &'static [(i64, i64)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    
    let mut map: Vec<Vec<u8>> = read_lines("input.txt").into_iter().map(|line| 
        line.into_bytes()
    ).collect();
    
    let (ix, iy) = (0..map.len()).flat_map(|i| (0..map[i].len()).map(move |j| (i, j)))
        .filter(|(i, j)| map[*i][*j] == b'^').next().unwrap();
    let ix = ix as i64;
    let iy = iy as i64;
    
    let mut obs = BTreeMap::new();
    let (mut x, mut y, mut d) = (ix, iy, 0);
    loop {
        let nx = x + DIRECTIONS[d].0;
        let ny = y + DIRECTIONS[d].1;
        match (usize::try_from(nx).ok().zip(usize::try_from(ny).ok())
            .and_then(|(nx, ny)| map.get(nx).and_then(|row| row.get(ny)))) {
                Some(b'.') | Some(b'^') => {
                    if !obs.contains_key(&(nx, ny)) {
                        map[nx as usize][ny as usize] = b'#';
                        let f = test_loop(&map, ix, iy);
                        obs.insert((nx, ny), f);
                        map[nx as usize][ny as usize] = b'.';
                    }
                    (x, y) = (nx, ny)
                },
                Some(b'#') => d = (d+1) % 4,
                None => break,
                _ => panic!()
            }
    }
    
    println!("{}", obs.values().filter(|&f| *f).count());
    /*
    for e in obs {
        if e.1 {
            println!("{} {}", e.0.0, e.0.1);
        }
    } */

}

fn test_loop(map: &Vec<Vec<u8>>, x: i64, y: i64) -> bool {
    let (mut x, mut y, mut d) = (x, y, 0);
    let mut visited = BTreeSet::new();
    loop {
        if visited.contains(&(x, y, d)) {
            return true;
        }
        visited.insert((x, y, d));
        let nx = x + DIRECTIONS[d].0;
        let ny = y + DIRECTIONS[d].1;
        match (usize::try_from(nx).ok().zip(usize::try_from(ny).ok())
            .and_then(|(nx, ny)| map.get(nx).and_then(|row| row.get(ny)))) {
                Some(b'.') | Some(b'^') => {
                    (x, y) = (nx, ny)
                },
                Some(b'#') => d = (d+1) % 4,
                None => break,
                _ => panic!()
            }
    }
    false
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}