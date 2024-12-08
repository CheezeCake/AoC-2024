use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

fn within_bounds(p: &Point, map: &[Vec<char>]) -> bool {
    p.y >= 0 && (p.y as usize) < map.len() && p.x >= 0 && (p.x as usize) < map[p.y as usize].len()
}

fn main() {
    let map: Vec<Vec<_>> = io::stdin()
        .lines()
        .map(|line| line.expect("error reading input").chars().collect())
        .collect();
    let mut antennas = HashMap::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != '.' {
                antennas.entry(map[y][x]).or_insert(Vec::new()).push(Point {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }

    let mut antinodes = HashSet::new();
    let mut antinodes_with_harmonics = HashSet::new();

    for (_, antennas) in antennas {
        for i in 0..antennas.len() - 1 {
            for j in i + 1..antennas.len() {
                let (a, b) = (antennas[i], antennas[j]);
                let delta = Point {
                    x: a.x - b.x,
                    y: a.y - b.y,
                };

                let antinode = Point {
                    x: a.x + delta.x,
                    y: a.y + delta.y,
                };
                if within_bounds(&antinode, &map) {
                    antinodes.insert(antinode);
                }
                let mut antinode = a;
                while within_bounds(&antinode, &map) {
                    antinodes_with_harmonics.insert(antinode);
                    antinode.x += delta.x;
                    antinode.y += delta.y;
                }

                let antinode = Point {
                    x: b.x - delta.x,
                    y: b.y - delta.y,
                };
                if within_bounds(&antinode, &map) {
                    antinodes.insert(antinode);
                }
                let mut antinode = b;
                while within_bounds(&antinode, &map) {
                    antinodes_with_harmonics.insert(antinode);
                    antinode.x -= delta.x;
                    antinode.y -= delta.y;
                }
            }
        }
    }

    println!("part 1: {}", antinodes.len());
    println!("part 2: {}", antinodes_with_harmonics.len());
}
