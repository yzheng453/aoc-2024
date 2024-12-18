use std::{collections::{BTreeMap, BTreeSet, VecDeque}, fs::read_to_string};
use std::ops::Bound::Included;

static DIRECTIONS: &'static [(i64, i64)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

static N: usize = 71;

fn main() {

    let corrupted: Vec<(usize, usize)> = read_lines("input.txt").into_iter().map(|s| {
        let mut i = s.split(',');
        let x = i.next().unwrap().parse().unwrap();
        let y = i.next().unwrap().parse().unwrap();
        (x, y)
    }).collect();
    
    let mut map = vec![vec![false; N]; N];
    for (x, y) in corrupted {
        map[x][y] = true;
    }
    let mut dist = vec![vec![5000; N]; N];
    let mut queue = VecDeque::new();
    dist[0][0] = 0;
    queue.push_back((0, 0));
    while let Some((x, y)) = queue.pop_front() {
        for &(dx, dy) in DIRECTIONS {
            let nx = x + dx;
            let ny = y + dy;
            if get_from_map(nx, ny, &map) == Some(false) {
                if dist[x as usize][y as usize] + 1 < dist[nx as usize][ny as usize] {
                    dist[nx as usize][ny as usize] = dist[x as usize][y as usize] + 1;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    for row in map.iter() {
        for d in row {
            print!("{} ", if *d {'#'} else {'.'});
        }
        println!("");
    }

    for row in dist.iter() {
        for d in row {
            print!("{} ", d);
        }
        println!("");
    }
    println!("{}", dist[N-1][N-1]);
}

fn get_from_map(x: i64, y: i64, map: &Vec<Vec<bool>>) -> Option<bool> {
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