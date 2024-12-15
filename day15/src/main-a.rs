use std::{collections::{BTreeMap, BTreeSet}, fs::read_to_string};

static DIRECTIONS: &'static [(i64, i64)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {

    let mut map = Vec::new();
    let mut instructions = Vec::new();
    let mut mode = 0;

    for line in read_lines("input.txt") {
        if line.len() == 0 {
            mode = 1;
        }
        match mode {
            0 => {
                map.push(line.into_bytes());
            },
            _ => {
                instructions.append(&mut line.into_bytes());
            }
        }
    }
    let (mut x, mut y, _) = map.iter().enumerate().flat_map(|(x, row)| row.iter().enumerate().filter(|b| *b.1 == b'@').map(move |(y, c)| (x as i64, y as i64, c))).next().unwrap();
    map[x as usize][y as usize] = b'.';
    
    for i in instructions {
        let d = match i {
            b'^' => 0,
            b'>' => 1,
            b'v' => 2,
            b'<' => 3,
            _ => panic!()
        };
        let (dx, dy) = DIRECTIONS[d];
        let (nx, ny) = (x + dx, y + dy);
        match get_from_map(nx, ny, &map) {
            Some(b'#') | None => {
                // Do nothing
            },
            Some(b'O') => {
                // Push
                let (mut cx, mut cy) = (nx, ny);
                loop {
                    (cx, cy) = (cx + dx, cy + dy);
                    match get_from_map(cx, cy, &map) {
                        Some(b'O') => {
                            // Nothing, continue to loop
                        },
                        Some(b'#') | None => {
                            // Stop, do nothing.
                            break;
                        },
                        Some(b'.') => {
                            // Empty space
                            map[cx as usize][cy as usize] = b'O';
                            map[nx as usize][ny as usize] = b'.';
                            (x, y) = (nx, ny);
                            break;
                        },
                        _ => panic!()
                    }
                }
            },
            Some(b'.') => {
                (x, y) = (nx, ny);
            }
            _ => panic!()
        }
    }
    
    let sum: i64 = map.iter().enumerate().flat_map(|(x, row)| row.iter().enumerate().filter(|(_, u)| **u == b'O').map(move |(y, _)| 100 * (x as i64) + y as i64)).sum();
    println!("{}", sum);
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