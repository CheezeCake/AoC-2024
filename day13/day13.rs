use std::io;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
struct ClawMachine {
    button_a_delta: Point,
    button_b_delta: Point,
    prize: Point,
}

#[derive(Debug)]
struct ParseClawMachineError;

fn parse_button(s: &str) -> Result<Point, ParseClawMachineError> {
    let (_, ops) = s.split_once(": ").ok_or(ParseClawMachineError)?;
    let (x_op, y_op) = ops.split_once(", ").ok_or(ParseClawMachineError)?;
    if x_op.len() < 2 || y_op.len() < 2 {
        Err(ParseClawMachineError)
    } else {
        Ok(Point {
            x: x_op[2..]
                .parse::<usize>()
                .map_err(|_| ParseClawMachineError)?,
            y: y_op[2..]
                .parse::<usize>()
                .map_err(|_| ParseClawMachineError)?,
        })
    }
}

fn parse_prize(s: &str) -> Result<Point, ParseClawMachineError> {
    let (_, vals) = s.split_once(": ").ok_or(ParseClawMachineError)?;
    let (x, y) = vals.split_once(", ").ok_or(ParseClawMachineError)?;
    let (_, x) = x.split_once('=').ok_or(ParseClawMachineError)?;
    let (_, y) = y.split_once('=').ok_or(ParseClawMachineError)?;
    Ok(Point {
        x: x.parse::<usize>().map_err(|_| ParseClawMachineError)?,
        y: y.parse::<usize>().map_err(|_| ParseClawMachineError)?,
    })
}

impl FromStr for ClawMachine {
    type Err = ParseClawMachineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        if lines.len() != 3 {
            Err(ParseClawMachineError)
        } else {
            Ok(ClawMachine {
                button_a_delta: parse_button(lines[0])?,
                button_b_delta: parse_button(lines[1])?,
                prize: parse_prize(lines[2])?,
            })
        }
    }
}

fn solve(machine: &ClawMachine, max_presses: usize) -> Option<usize> {
    let (x, y) = (machine.prize.x as f64, machine.prize.y as f64);
    let (dax, day) = (
        machine.button_a_delta.x as f64,
        machine.button_a_delta.y as f64,
    );
    let (dbx, dby) = (
        machine.button_b_delta.x as f64,
        machine.button_b_delta.y as f64,
    );
    let b = (y - (day * x / dax)) / (dby - (dbx * day / dax));
    let a = (x / dax) - (b * dbx / dax);

    let a = a.round() as usize;
    let b = b.round() as usize;
    if a <= max_presses
        && b <= max_presses
        && a * machine.button_a_delta.x + b * machine.button_b_delta.x == machine.prize.x
        && a * machine.button_a_delta.y + b * machine.button_b_delta.y == machine.prize.y
    {
        Some(a * 3 + b)
    } else {
        None
    }
}

fn main() {
    let input = io::read_to_string(io::stdin()).expect("error reading input");
    let machines: Vec<_> = input
        .split("\n\n")
        .map(|s| {
            s.parse::<ClawMachine>()
                .expect("error parsing claw machine info")
        })
        .collect();

    println!(
        "part 1: {}",
        machines
            .iter()
            .filter_map(|machine| solve(machine, 100))
            .sum::<usize>()
    );
    println!(
        "part 2: {}",
        machines
            .iter()
            .map(
                |&ClawMachine {
                     prize,
                     button_a_delta,
                     button_b_delta,
                 }| ClawMachine {
                    prize: Point {
                        x: prize.x + 10000000000000,
                        y: prize.y + 10000000000000
                    },
                    button_a_delta,
                    button_b_delta
                }
            )
            .filter_map(|machine| solve(&machine, usize::MAX))
            .sum::<usize>()
    );
}
