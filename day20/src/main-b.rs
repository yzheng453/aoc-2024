use std::{collections::{BTreeMap, BTreeSet, VecDeque}, fs::read_to_string};

static DIRECTIONS: &'static [(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    let map: Vec<Vec<u8>> = read_lines("input.txt").into_iter().map(|l| l.into_bytes()).collect();
    let s = map.iter().enumerate().flat_map(|(x, row)| row.iter().enumerate().map(move |(y, u)| (x, y, u)))
        .filter(|(x, y, u)| **u == b'S').next().unwrap();
    
    let mut distances: BTreeMap<(i32, i32), i32> = BTreeMap::new();
    let mut track = Vec::new();
    let (mut x, mut y) = (s.0 as i32, s.1 as i32);
    loop {
        distances.insert((x as i32, y as i32), track.len() as i32);
        track.push((x, y));

        match try_move(x, y, &map).into_iter().filter(|&(nx, ny, u)| {
            (u == b'.' || u == b'E') && !distances.contains_key(&(nx, ny))
        }).next() {
            Some((nx, ny, _)) => (x, y) = (nx, ny),
            None => break,
        }
    }
    
    let distances = &distances;
    
    let cheats: Vec<((i32, i32), (i32, i32), i32)> = track.iter().flat_map(|&(x, y)|
        find_best_cheat(x, y, &distances, distances[&(x, y)])
    ).filter(|x| x.2 > 0).collect();

    let mut histo: BTreeMap<i32, Vec<((i32, i32), (i32, i32), i32)>> = BTreeMap::new();
    for &c in cheats.iter() {
        histo.entry(c.2)
            .and_modify(|list| list.push(c))
            .or_insert(vec![c; 1]);
    }
    
    for entry in histo {
        println!("{} {}", entry.0, entry.1.len());
        println!("{:?}", entry.1);
    }
    println!("{}", cheats.iter().filter(|x| x.2 >= 100).count());
}

fn find_best_cheat(x: i32, y: i32, distances: &BTreeMap<(i32, i32), i32>, starting_distance: i32) -> Vec<((i32, i32), (i32, i32), i32)> {
    let mut cheat_distances = Vec::new();
    for p in 0..=20i32 {
        for dx in -p..=p {
            let dy = p - dx.abs();
            cheat_distances.extend(distances.get(&(x + dx, y + dy)).map(|&d| ((x, y), (x + dx, y + dy), d - p - starting_distance)));
            if dy > 0 {
                cheat_distances.extend(distances.get(&(x + dx, y - dy)).map(|&d| ((x, y), (x + dx, y + dy), d - p - starting_distance)))
            };
        }
    }
    cheat_distances
}

fn try_move(x: i32, y: i32, map: &Vec<Vec<u8>>) -> Vec<(i32, i32, u8)> {
    DIRECTIONS.iter().flat_map(|d| {
        let nx = x as i32 + d.0;
        let ny = y as i32 + d.1;
        return get_from_map(nx, ny, map).map(|u| (nx, ny, u));
    }).collect()
}

fn get_from_map(x: i32, y: i32, map: &Vec<Vec<u8>>) -> Option<u8> {
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