use std::io;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Box,
    Robot,
    WideBoxL,
    WideBoxR,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn up_or_down(&self) -> bool {
        match self {
            Self::Up | Self::Down => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn next_position(self: &Self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

fn find_robot(warehouse: &[Vec<Tile>]) -> Option<Point> {
    for y in 0..warehouse.len() {
        for x in 0..warehouse[y].len() {
            if warehouse[y][x] == Tile::Robot {
                return Some(Point {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }
    None
}

fn can_move_object(pos: &Point, dir: Direction, warehouse: &[Vec<Tile>]) -> bool {
    match warehouse[pos.y as usize][pos.x as usize] {
        Tile::Empty => true,
        Tile::Wall => false,
        Tile::WideBoxL if dir.up_or_down() => {
            can_move_object(&pos.next_position(dir), dir, warehouse)
                && can_move_object(
                    &pos.next_position(Direction::Right).next_position(dir),
                    dir,
                    warehouse,
                )
        }
        Tile::WideBoxR if dir.up_or_down() => {
            can_move_object(&pos.next_position(dir), dir, warehouse)
                && can_move_object(
                    &pos.next_position(Direction::Left).next_position(dir),
                    dir,
                    warehouse,
                )
        }
        _ => can_move_object(&pos.next_position(dir), dir, warehouse),
    }
}

fn move_object(pos: &Point, replacement: Tile, dir: Direction, warehouse: &mut [Vec<Tile>]) {
    match warehouse[pos.y as usize][pos.x as usize] {
        Tile::Empty => warehouse[pos.y as usize][pos.x as usize] = replacement,
        Tile::WideBoxL if dir.up_or_down() => {
            let right_half = pos.next_position(Direction::Right);
            move_object(&pos.next_position(dir), Tile::WideBoxL, dir, warehouse);
            move_object(&right_half.next_position(dir), Tile::WideBoxR, dir, warehouse);
            warehouse[pos.y as usize][pos.x as usize] = replacement;
            warehouse[right_half.y as usize][right_half.x as usize] = Tile::Empty;
        }
        Tile::WideBoxR if dir.up_or_down() => {
            let left_half = pos.next_position(Direction::Left);
            move_object(&pos.next_position(dir), Tile::WideBoxR, dir, warehouse);
            move_object(&left_half.next_position(dir), Tile::WideBoxL, dir, warehouse);
            warehouse[pos.y as usize][pos.x as usize] = replacement;
            warehouse[left_half.y as usize][left_half.x as usize] = Tile::Empty;
        }
        object => {
            move_object(&pos.next_position(dir), object, dir, warehouse);
            warehouse[pos.y as usize][pos.x as usize] = replacement;
        }
    }
}

fn gps_sum(warehouse: &[Vec<Tile>]) -> usize {
    (0..warehouse.len())
        .map(|y| {
            (0..warehouse[y].len())
                .map(|x| match warehouse[y][x] {
                    Tile::Box | Tile::WideBoxL => y * 100 + x,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn run(directions: &[Direction], warehouse: &mut [Vec<Tile>]) {
    let mut robot_pos = find_robot(&warehouse).expect("no robot");
    for &dir in directions {
        if can_move_object(&robot_pos, dir, &warehouse) {
            move_object(&robot_pos, Tile::Empty, dir, warehouse);
            robot_pos = robot_pos.next_position(dir);
        }
    }
}

fn main() {
    let input = io::read_to_string(io::stdin()).expect("error reading input");
    let (warehouse, directions) = input.split_once("\n\n").expect("error parsing input");
    let mut warehouse: Vec<Vec<_>> = warehouse
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    'O' => Tile::Box,
                    '@' => Tile::Robot,
                    _ => panic!("unexpected character in warehouse: '{}'", c),
                })
                .collect()
        })
        .collect();
    let directions: Vec<_> = directions
        .lines()
        .map(|line| {
            line.chars().map(|c| match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => panic!("unexpected direction: '{}'", c),
            })
        })
        .flatten()
        .collect();
    let mut wide_warehouse: Vec<Vec<_>> = warehouse
        .iter()
        .map(|row| {
            row.iter()
                .map(|t| match t {
                    Tile::Empty => [Tile::Empty, Tile::Empty].iter(),
                    Tile::Wall => [Tile::Wall, Tile::Wall].iter(),
                    Tile::Box => [Tile::WideBoxL, Tile::WideBoxR].iter(),
                    Tile::Robot => [Tile::Robot, Tile::Empty].iter(),
                    _ => unreachable!(),
                })
                .flatten()
                .cloned()
                .collect()
        })
        .collect();

    run(&directions, &mut warehouse);
    println!("part 1: {}", gps_sum(&warehouse));

    run(&directions, &mut wide_warehouse);
    println!("part 2: {}", gps_sum(&wide_warehouse));
}
