use std::collections::HashSet;
use std::io;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Ord, PartialOrd)]
struct Point {
    x: isize,
    y: isize,
}

const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };
const TOP: Point = Point { x: 0, y: -1 };
const BOTTOM: Point = Point { x: 0, y: 1 };

fn within_bounds(p: &Point, map: &[Vec<char>]) -> bool {
    p.y >= 0 && (p.y as usize) < map.len() && p.x >= 0 && (p.x as usize) < map[p.y as usize].len()
}

fn map_region(pos: &Point, plant_type: char, map: &[Vec<char>], visited: &mut HashSet<Point>) {
    if within_bounds(pos, map)
        && map[pos.y as usize][pos.x as usize] == plant_type
        && !visited.contains(pos)
    {
        visited.insert(*pos);
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in directions {
            let next_pos = Point {
                x: pos.x + dx,
                y: pos.y + dy,
            };
            map_region(&next_pos, plant_type, map, visited);
        }
    }
}

fn is_edge(pos: &Point, delta: &Point, map: &[Vec<char>]) -> bool {
    let adj = Point {
        x: pos.x + delta.x,
        y: pos.y + delta.y,
    };
    !within_bounds(&adj, map)
        || map[adj.y as usize][adj.x as usize] != map[pos.y as usize][pos.x as usize]
}

fn region_perimeter(region: &HashSet<Point>, map: &[Vec<char>]) -> usize {
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    region
        .iter()
        .map(|p| {
            directions
                .iter()
                .filter(|(dx, dy)| is_edge(p, &Point { x: *dx, y: *dy }, map))
                .count()
        })
        .sum::<usize>()
}

fn region_sides(region: &HashSet<Point>, map: &[Vec<char>]) -> usize {
    let mut edges: [(Point, Point, HashSet<Point>); 4] = [
        (LEFT, BOTTOM, HashSet::new()),
        (RIGHT, BOTTOM, HashSet::new()),
        (TOP, RIGHT, HashSet::new()),
        (BOTTOM, RIGHT, HashSet::new()),
    ];

    let mut sorted_region: Vec<Point> = region.iter().cloned().collect();
    sorted_region.sort();

    let mut count = 0;
    for p in sorted_region {
        for (edge_delta, dir, set) in &mut edges {
            if !is_edge(&p, edge_delta, map) || set.contains(&p) {
                continue;
            }
            let mut pos = p;
            while region.contains(&pos)
                && !region.contains(&Point {
                    x: pos.x + edge_delta.x,
                    y: pos.y + edge_delta.y,
                })
            {
                set.insert(pos);
                pos.x += dir.x;
                pos.y += dir.y;
            }
            count += 1;
        }
    }
    count
}

fn main() {
    let map: Vec<Vec<_>> = io::stdin()
        .lines()
        .map(|line| line.expect("error reading input").chars().collect())
        .collect();

    let mut total_price1 = 0;
    let mut total_price2 = 0;

    let mut visited = HashSet::new();
    for y in 0..map.len() {
        for x in 0..map.len() {
            let p = Point {
                x: x as isize,
                y: y as isize,
            };
            if visited.contains(&p) {
                continue;
            }

            let mut region = HashSet::new();
            map_region(&p, map[y][x], &map, &mut region);

            let area = region.len();
            let perimeter = region_perimeter(&region, &map);
            let sides = region_sides(&region, &map);

            total_price1 += area * perimeter;
            total_price2 += area * sides;

            for x in region {
                visited.insert(x);
            }
        }
    }

    println!("part 1: {}", total_price1);
    println!("part 2: {}", total_price2);
}
