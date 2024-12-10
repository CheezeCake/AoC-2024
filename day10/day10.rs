use std::collections::HashSet;
use std::io;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

fn within_bounds(p: &Point, map: &[Vec<u8>]) -> bool {
    p.y >= 0 && (p.y as usize) < map.len() && p.x >= 0 && (p.x as usize) < map[p.y as usize].len()
}

fn hiking_trails(pos: &Point, map: &[Vec<u8>], trailtail: &mut HashSet<Point>) -> usize {
    if map[pos.y as usize][pos.x as usize] == 9 {
        trailtail.insert(*pos);
        return 1;
    }

    let mut trails = 0;
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dx, dy) in directions {
        let next_pos = Point {
            x: pos.x + dx,
            y: pos.y + dy,
        };
        if within_bounds(&next_pos, map)
            && map[next_pos.y as usize][next_pos.x as usize]
                == map[pos.y as usize][pos.x as usize] + 1
        {
            trails += hiking_trails(&next_pos, map, trailtail);
        }
    }

    trails
}

fn main() {
    let map: Vec<Vec<_>> = io::stdin()
        .lines()
        .map(|line| {
            line.expect("error reading input")
                .bytes()
                .map(|c| c - b'0')
                .collect()
        })
        .collect();

    let mut score_sum = 0;
    let mut rating_sum = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 0 {
                let trailhead = Point {
                    x: x as isize,
                    y: y as isize,
                };
                let mut trailtails = HashSet::new();
                let x = hiking_trails(&trailhead, &map, &mut trailtails);
                score_sum += trailtails.len();
                rating_sum += x;
            }
        }
    }
    println!("part 1: {}", score_sum);
    println!("part 2: {}", rating_sum);
}
