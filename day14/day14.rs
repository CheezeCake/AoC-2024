use std::collections::HashSet;
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

#[derive(Copy, Clone, Debug)]
struct Robot {
    p: Point,
    v: Point,
}

#[derive(Debug)]
struct ParseRobotError;

impl FromStr for Robot {
    type Err = ParseRobotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(' ').ok_or(ParseRobotError)?;
        let (_, p) = p.split_once('=').ok_or(ParseRobotError)?;
        let (_, v) = v.split_once('=').ok_or(ParseRobotError)?;
        Ok(Robot {
            p: p.parse::<Point>().map_err(|_| ParseRobotError)?,
            v: v.parse::<Point>().map_err(|_| ParseRobotError)?,
        })
    }
}

fn advance(robot: Robot, map_width: usize, map_height: usize) -> Robot {
    let x = (robot.p.x + robot.v.x) % (map_width as isize);
    let y = (robot.p.y + robot.v.y) % (map_height as isize);
    Robot {
        p: Point {
            x: if x < 0 { (map_width as isize) + x } else { x },
            y: if y < 0 { (map_height as isize) + y } else { y },
        },
        v: robot.v,
    }
}

fn longest_line(robots: &[Robot], map_width: usize, map_height: usize) -> usize {
    let mut map = vec![vec![false; map_width]; map_height];
    for robot in robots {
        map[robot.p.y as usize][robot.p.x as usize] = true;
    }
    let mut longest = 0;
    for y in 0..map_height {
        let mut x = 0;
        while x < map_width {
            if map[y][x] {
                let mut i = 1;
                while x + i < map_width && map[y][x + i] {
                    i += 1
                }
                longest = longest.max(i);
                x += i;
            } else {
                x += 1;
            }
        }
    }
    longest
}

fn display(robots: &[Robot], map_width: usize, map_height: usize) {
    let positions: HashSet<Point> = robots.iter().map(|robot| robot.p).collect();
    for y in 0..map_height {
        for x in 0..map_width {
            if positions.contains(&Point { x: x as isize, y: y as isize }) {
                print!("{}", '#');
            } else {
                print!("{}", '.');
            }
        }
        println!();
    }
    println!();
}

fn quadrant_count(
    robots: &[Robot],
    map_width: usize,
    map_height: usize,
) -> (usize, usize, usize, usize) {
    robots
        .iter()
        .map(|robot| {
            let w_mid = (map_width as isize) / 2;
            let h_mid = (map_height as isize) / 2;
            if robot.p.x < w_mid {
                if robot.p.y < h_mid {
                    (1, 0, 0, 0)
                } else if robot.p.y > h_mid {
                    (0, 1, 0, 0)
                } else {
                    (0, 0, 0, 0)
                }
            } else if robot.p.x > w_mid {
                if robot.p.y < h_mid {
                    (0, 0, 1, 0)
                } else if robot.p.y > h_mid {
                    (0, 0, 0, 1)
                } else {
                    (0, 0, 0, 0)
                }
            } else {
                (0, 0, 0, 0)
            }
        })
        .reduce(|acc, e| (acc.0 + e.0, acc.1 + e.1, acc.2 + e.2, acc.3 + e.3))
        .unwrap()
}

fn main() {
    let mut robots: Vec<_> = io::stdin()
        .lines()
        .map(|line| {
            line.expect("error reading input")
                .parse::<Robot>()
                .expect("error parsing robot")
        })
        .collect();

    let map_width = 101;
    let map_height = 103;

    for _ in 1..=100 {
        for robot in &mut robots {
            *robot = advance(*robot, map_width, map_height);
        }
    }

    let (tl, bl, tr, br) = quadrant_count(&robots, map_width, map_height);
    println!("part 1: {}", tl * tr * bl * br);

    for i in 101.. {
        for robot in &mut robots {
            *robot = advance(*robot, map_width, map_height);
        }
        if longest_line(&robots, map_width, map_height) > 10 {
            display(&robots, map_width, map_height);
            println!("part 2: {}", i);
            break;
        }
    }
}
