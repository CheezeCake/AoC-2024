use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(ParsePointError)?;
        Ok(Point {
            x: x.parse::<isize>().map_err(|_| ParsePointError)?,
            y: y.parse::<isize>().map_err(|_| ParsePointError)?,
        })
    }
}

fn within_bounds(p: &Point, mem_size: usize) -> bool {
    p.x >= 0 && (p.x as usize) <= mem_size && p.y >= 0 && (p.y as usize) <= mem_size
}

fn min_steps(
    start: Point,
    end: Point,
    mem_size: usize,
    corrupted: &HashSet<Point>,
) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();

    q.push_back((start, 0));
    visited.insert(start);

    while let Some((pos, steps)) = q.pop_front() {
        if pos == end {
            return Some(steps);
        }
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in directions {
            let next_pos = Point {
                x: pos.x + dx,
                y: pos.y + dy,
            };
            if within_bounds(&next_pos, mem_size)
                && !visited.contains(&next_pos)
                && !corrupted.contains(&next_pos)
            {
                q.push_back((next_pos, steps + 1));
                visited.insert(next_pos);
            }
        }
    }
    None
}

fn main() {
    let bytes: Vec<_> = io::stdin()
        .lines()
        .map(|line| {
            line.expect("error reading input")
                .parse::<Point>()
                .expect("error parsing point")
        })
        .collect();

    let mem_size = 70;
    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: mem_size,
        y: mem_size,
    };
    let mut corrupted: HashSet<_> = bytes.iter().take(1024).cloned().collect();

    println!(
        "part 1: {}",
        min_steps(start, end, mem_size as usize, &corrupted).unwrap()
    );

    for i in 1025..bytes.len() {
        corrupted.insert(bytes[i]);
        if min_steps(start, end, mem_size as usize, &corrupted).is_none() {
            println!("part 2: {},{}", bytes[i].x, bytes[i].y);
            break;
        }
    }
}
