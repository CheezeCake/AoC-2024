use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

type Tile = char;

fn find_tile(tile: Tile, map: &[Vec<Tile>]) -> Option<Point> {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == tile {
                return Some(Point {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }
    None
}

fn within_bounds(p: &Point, map: &[Vec<Tile>]) -> bool {
    p.y >= 0 && (p.y as usize) < map.len() && p.x >= 0 && (p.x as usize) < map[p.y as usize].len()
}

fn bfs(start: &Point, end: &Point, map: &[Vec<Tile>], times: &mut HashMap<Point, usize>) {
    if map[start.y as usize][start.x as usize] == '#' {
        return;
    }

    let mut q = VecDeque::new();
    let mut visited = HashSet::new();

    q.push_back((*start, 0));
    visited.insert(*start);
    times.insert(*start, 0);

    while let Some((pos, time)) = q.pop_front() {
        if pos == *end {
            continue;
        }

        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in directions {
            let next_pos = Point {
                x: pos.x + dx,
                y: pos.y + dy,
            };
            if within_bounds(&next_pos, map)
                && !visited.contains(&next_pos)
                && map[next_pos.y as usize][next_pos.x as usize] != '#'
            {
                q.push_back((next_pos, time + 1));
                visited.insert(next_pos);
                times.insert(next_pos, time + 1);
            }
        }
    }
}

fn cheat_count(max_cheat_len: usize, end_time: usize, from_start_times: &HashMap<Point, usize>) -> usize {
    let mut n = 0;
    for (cheat_start, t1) in from_start_times {
        if t1 + 100 > end_time {
            continue;
        }
        for (cheat_end, t) in from_start_times {
            let t2 = end_time - t;
            let cheat_len = cheat_start.distance(&cheat_end);
            if cheat_len <= max_cheat_len {
                let time = t1 + cheat_len + t2;
                if time + 100 <= end_time {
                    n += 1;
                }
            }
        }
    }
    n
}

fn main() {
    let map: Vec<Vec<_>> = io::stdin()
        .lines()
        .map(|line| line.expect("error reading input").chars().collect())
        .collect();

    let start = find_tile('S', &map).expect("start tile not found");
    let end = find_tile('E', &map).expect("end tile not found");

    let mut from_start_times = HashMap::new();
    bfs(&start, &end, &map, &mut from_start_times);
    let end_time = *from_start_times.get(&end).unwrap();

    println!("part 1: {}", cheat_count(2, end_time, &from_start_times));
    println!("part 2: {}", cheat_count(20, end_time, &from_start_times));
}
