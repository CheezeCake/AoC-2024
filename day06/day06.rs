use std::collections::HashSet;
use std::io;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Tile {
    Space,
    Obstruction,
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn get_tile(pos: &Point, map: &[Vec<Tile>]) -> Option<Tile> {
    if pos.y >= 0
        && (pos.y as usize) < map.len()
        && pos.x >= 0
        && (pos.x as usize) < map[pos.y as usize].len()
    {
        Some(map[pos.y as usize][pos.x as usize])
    } else {
        None
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct Guard {
    pos: Point,
    dir: Direction,
}

impl Guard {
    fn next_pos(&self) -> Point {
        match self.dir {
            Direction::Up => Point {
                x: self.pos.x,
                y: self.pos.y - 1,
            },
            Direction::Down => Point {
                x: self.pos.x,
                y: self.pos.y + 1,
            },
            Direction::Left => Point {
                x: self.pos.x - 1,
                y: self.pos.y,
            },
            Direction::Right => Point {
                x: self.pos.x + 1,
                y: self.pos.y,
            },
        }
    }
}

fn find_loop(guard: &Guard, map: &[Vec<Tile>], guard_states: &mut HashSet<Guard>) -> bool {
    if guard_states.contains(guard) {
        return true;
    }

    guard_states.insert(*guard);

    let next_pos = guard.next_pos();
    let ret = match get_tile(&next_pos, &map) {
        Some(Tile::Space) => find_loop(
            &Guard {
                pos: next_pos,
                dir: guard.dir,
            },
            map,
            guard_states,
        ),
        Some(Tile::Obstruction) => find_loop(
            &Guard {
                pos: guard.pos,
                dir: guard.dir.turn_right(),
            },
            map,
            guard_states,
        ),
        None => false,
    };

    guard_states.remove(guard);

    ret
}

fn main() {
    let mut guard = Guard {
        pos: Point { x: 0, y: 0 },
        dir: Direction::Up,
    };
    let mut map: Vec<Vec<Tile>> = io::stdin()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.expect("error reading input")
                .chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Tile::Space,
                    '#' => Tile::Obstruction,
                    _ => {
                        guard = Guard {
                            pos: Point {
                                x: x as isize,
                                y: y as isize,
                            },
                            dir: match c {
                                '^' => Direction::Up,
                                'v' => Direction::Down,
                                '<' => Direction::Left,
                                '>' => Direction::Right,
                                _ => panic!("invalid character in map"),
                            },
                        };
                        Tile::Space
                    }
                })
                .collect()
        })
        .collect();

    let mut guard_states = HashSet::new();
    let mut visited = HashSet::new();
    let mut loops = 0;
    loop {
        guard_states.insert(guard);
        visited.insert(guard.pos);

        let next_pos = guard.next_pos();
        match get_tile(&next_pos, &map) {
            Some(Tile::Space) => {
                map[next_pos.y as usize][next_pos.x as usize] = Tile::Obstruction;
                if !visited.contains(&next_pos)
                    && find_loop(
                        &Guard {
                            pos: guard.pos,
                            dir: guard.dir.turn_right(),
                        },
                        &map,
                        &mut guard_states,
                    )
                {
                    loops += 1;
                }
                map[next_pos.y as usize][next_pos.x as usize] = Tile::Space;

                guard.pos = next_pos;
            }
            Some(Tile::Obstruction) => guard.dir = guard.dir.turn_right(),
            None => break,
        }
    }

    println!("part 1: {}", visited.len());
    println!("part 2: {}", loops);
}
