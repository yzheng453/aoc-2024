use std::{collections::BTreeSet, fs::read_to_string};

const DIRECTIONS: &'static [(i64, i64)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    
    let map: Vec<Vec<u8>> = read_lines("input.txt").into_iter().map(|line| 
        line.into_bytes()
    ).collect();
    
    let (x, y) = (0..map.len()).flat_map(|i| (0..map[i].len()).map(move |j| (i, j)))
        .filter(|(i, j)| map[*i][*j] == b'^').next().unwrap();
    
    let mut x = x as i64;
    let mut y = y as i64;
    
    let mut d = 0;
    
    let mut visited: BTreeSet<(i64, i64)> = BTreeSet::new();

    loop {
        visited.insert((x, y));
        let nx = x + DIRECTIONS[d].0;
        let ny = y + DIRECTIONS[d].1;
        match usize::try_from(nx).ok().zip(usize::try_from(ny).ok())
            .and_then(|(nx, ny)| map.get(nx).and_then(|row| row.get(ny))) {
                Some(b'.') | Some(b'^') => (x, y) = (nx, ny),
                Some(b'#') => d = (d+1) % 4,
                None => break,
                _ => panic!()
            }
    }
    
    print!("{}", visited.len());

}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}