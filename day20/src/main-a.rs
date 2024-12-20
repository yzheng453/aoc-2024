use std::{collections::{BTreeMap, BTreeSet, VecDeque}, fs::read_to_string};

static DIRECTIONS: &'static [(i64, i64)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    let map: Vec<Vec<u8>> = read_lines("input.txt").into_iter().map(|l| l.into_bytes()).collect();
    let s = map.iter().enumerate().flat_map(|(x, row)| row.iter().enumerate().map(move |(y, u)| (x, y, u)))
        .filter(|(x, y, u)| **u == b'S').next().unwrap();
    
    let mut distances: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    let mut track = Vec::new();
    let (mut x, mut y) = (s.0, s.1);
    loop {
        distances.insert((x, y), track.len());
        track.push((x, y));

        match try_move(x, y, &map).into_iter().filter(|&(nx, ny, u)| {
            (u == b'.' || u == b'E') && !distances.contains_key(&(nx, ny))
        }).next() {
            Some((nx, ny, _)) => (x, y) = (nx, ny),
            None => break,
        }
    }
    
    let mut shortcuts: Vec<usize> = track.iter().flat_map(|&(x, y)| 
        try_move(x, y, &map).into_iter()
            .filter(|&(_, _, u)| u == b'#')
            .flat_map(|(x1, y1, __)| try_move(x1, y1, &map).into_iter())
            .map(move |(x2, y2, u)| (x, y, x2, y2, u))
            .filter(|&(x, y, x2, y2, u)| (x, y) != (x2, y2) && (u == b'.' || u == b'E'))
            .flat_map(|(x, y, x2, y2, _)| if (distances[&(x2, y2)] > distances[&(x, y)] + 2) {Some(distances[&(x2, y2)] - distances[&(x, y)] - 2)}
                else { None })
    ).collect();
    shortcuts.sort();

    println!("{:?}", shortcuts);
    println!("{}", shortcuts.iter().filter(|x| **x >= 100).count());
}

fn try_move(x: usize, y: usize, map: &Vec<Vec<u8>>) -> Vec<(usize, usize, u8)> {
    DIRECTIONS.iter().flat_map(|d| {
        let nx = x as i64 + d.0;
        let ny = y as i64 + d.1;
        return get_from_map(nx, ny, map).map(|u| (nx as usize, ny as usize, u));
    }).collect()
}
 

fn get_from_map(x: i64, y: i64, map: &Vec<Vec<u8>>) -> Option<u8> {
    if let Ok(x) = usize::try_from(x) {
        if let Ok(y) = usize::try_from(y) {
            return map.get(x).and_then(|row| row.get(y).copied());
        }
    }
    return None;
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}