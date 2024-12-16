use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn next_position(&self, direction: Direction) -> Self {
        let (dx, dy) = match direction {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        };
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

fn find_tile(tile: char, map: &[Vec<char>]) -> Option<Point> {
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

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::West => Self::South,
            Self::East => Self::North,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::East => Self::South,
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Reindeer {
    position: Point,
    direction: Direction,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    score: usize,
    reindeer: Reindeer,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.reindeer.cmp(&other.reindeer))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn map_free_tile(position: Point, map: &[Vec<char>]) -> bool {
    if position.y < 0 || position.x < 0 {
        return false;
    }
    match map
        .get(position.y as usize)
        .and_then(|row| row.get(position.x as usize))
    {
        None | Some('#') => false,
        _ => true,
    }
}

fn best_path(
    start: Point,
    start_direction: Direction,
    end: Point,
    map: &[Vec<char>],
    prev: &mut HashMap<Reindeer, Vec<Reindeer>>,
) -> Option<(Reindeer, usize)> {
    let mut scores = HashMap::new();
    let mut pq = BinaryHeap::new();

    let reindeer = Reindeer {
        position: start,
        direction: start_direction,
    };
    scores.insert(reindeer, 0);
    prev.insert(reindeer, Vec::new());
    pq.push(State { score: 0, reindeer });

    while let Some(State { score, reindeer }) = pq.pop() {
        if reindeer.position == end {
            return Some((reindeer, score));
        }

        if score > *scores.get(&reindeer).unwrap_or(&usize::MAX) {
            continue;
        }

        let position = reindeer.position;
        let direction = reindeer.direction;
        let next = [
            (score + 1000, position, direction.turn_right()),
            (score + 1000, position, direction.turn_left()),
            (score + 1, position.next_position(direction), direction),
        ];

        for (score, position, direction) in next {
            let next_state = State {
                score,
                reindeer: Reindeer {
                    position,
                    direction,
                },
            };
            if !map_free_tile(position, map) {
                continue;
            }

            match next_state
                .score
                .cmp(scores.get(&next_state.reindeer).unwrap_or(&usize::MAX))
            {
                Ordering::Less => {
                    pq.push(next_state);
                    scores.insert(next_state.reindeer, next_state.score);
                    prev.insert(next_state.reindeer, vec![reindeer]);
                }
                Ordering::Equal => prev.get_mut(&next_state.reindeer).unwrap().push(reindeer),
                _ => {}
            }
        }
    }

    None
}

fn count_tiles_rec(
    reindeer: &Reindeer,
    prev: &HashMap<Reindeer, Vec<Reindeer>>,
    count: &mut HashSet<Point>,
) {
    count.insert(reindeer.position);
    if let Some(prev_reindeers) = prev.get(reindeer) {
        for prev_reindeer in prev_reindeers {
            count_tiles_rec(prev_reindeer, prev, count);
        }
    }
}

fn count_tiles(reindeer: &Reindeer, prev: &HashMap<Reindeer, Vec<Reindeer>>) -> usize {
    let mut count = HashSet::new();
    count_tiles_rec(reindeer, prev, &mut count);
    count.len()
}

fn main() {
    let map: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.expect("error reading input").chars().collect())
        .collect();

    let start = find_tile('S', &map).expect("no start tile");
    let end = find_tile('E', &map).expect("no end tile");

    let mut prev = HashMap::new();
    let (reindeer, score) =
        best_path(start, Direction::East, end, &map, &mut prev).expect("no path found");

    println!("part 1: {}", score);
    println!("part 2: {}", count_tiles(&reindeer, &prev));
}
