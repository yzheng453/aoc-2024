use std::{collections::{BTreeMap, BTreeSet}, fs::read_to_string};

const DIRECTIONS: &'static [(i64, i64)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    
    let map: Vec<Vec<u8>> = read_lines("input.txt").into_iter().map(|line| 
        line.into_bytes()
    ).collect();
    
    let mut cols = vec![BTreeSet::<i64>::new(); 150];
    let mut rows= vec![BTreeSet::<i64>::new(); 150];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == b'#' {
                cols[j].insert(i as i64);
                rows[i].insert(j as i64);
            }
        }
    }
    
    let (ix, iy) = (0..map.len()).flat_map(|i| (0..map[i].len()).map(move |j| (i, j)))
        .filter(|(i, j)| map[*i][*j] == b'^').next().unwrap();
    let ix = ix as i64;
    let iy = iy as i64;
    
    let mut obs = BTreeMap::new();
    let (mut x, mut y, mut d) = (ix, iy, 0);
    loop {
        let nx = x + DIRECTIONS[d].0;
        let ny = y + DIRECTIONS[d].1;
        match usize::try_from(nx).ok().zip(usize::try_from(ny).ok())
            .and_then(|(nx, ny)| map.get(nx).and_then(|row| row.get(ny))) {
                Some(b'.') | Some(b'^') => {
                    if !obs.contains_key(&(nx, ny)) {
                        cols[ny as usize].insert(nx);
                        rows[nx as usize].insert(ny);

                        let f = test_loop(&cols, &rows, ix, iy);
                        obs.insert((nx, ny), f);

                        cols[ny as usize].remove(&nx);
                        rows[nx as usize].remove(&ny);
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

fn test_loop(cols: &Vec<BTreeSet<i64>>, rows: &Vec<BTreeSet<i64>>, x: i64, y: i64) -> bool {
    let (mut x, mut y, mut d) = (x, y, 0);
    let mut visited = BTreeSet::new();
    loop {
        if visited.contains(&(x, y, d)) {
            return true;
        }
        visited.insert((x, y, d));
        
        
        match d {
            0 => match cols[y as usize].range(..x).next_back() {
                Some(&o) => x = o + 1,
                None => break
            }
            1 => match rows[x as usize].range(y..).next() {
                Some(&o) => y = o - 1,
                None => break
            }
            2 => match cols[y as usize].range(x..).next() {
                Some(&o) => x = o - 1,
                None => break
            }
            3 => match rows[x as usize].range(..y).next_back() {
                Some(&o) => y = o + 1,
                None => break
            }
            _ => panic!()
        }
        d = (d+1) % 4;
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