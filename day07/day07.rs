use std::io;
use std::str::FromStr;

struct Equation {
    result: u64,
    operands: Vec<u64>,
}

#[derive(Debug)]
struct ParseEquationError;

impl FromStr for Equation {
    type Err = ParseEquationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (result, operands) = s.split_once(": ").ok_or(ParseEquationError)?;
        let result = result.parse::<u64>().map_err(|_| ParseEquationError)?;
        let operands: Result<Vec<_>, _> = operands
            .split(' ')
            .map(|n| n.parse::<u64>().map_err(|_| ParseEquationError))
            .collect();
        Ok(Self {
            result,
            operands: operands?,
        })
    }
}

fn add(lhs: u64, rhs: u64) -> u64 {
    lhs + rhs
}

fn mul(lhs: u64, rhs: u64) -> u64 {
    lhs * rhs
}

fn concat(lhs: u64, rhs: u64) -> u64 {
    if rhs == 0 {
        lhs
    } else {
        let log = rhs.ilog10();
        lhs * 10_u64.pow(log + 1) + rhs
    }
}

type Operator = fn(u64, u64) -> u64;

fn solvable(current: u64, target: u64, operands: &[u64], operators: &[Operator]) -> bool {
    if operands.len() == 0 {
        current == target
    } else if current > target {
        false
    } else {
        operators
            .iter()
            .any(|op| solvable(op(current, operands[0]), target, &operands[1..], operators))
    }
}

impl Equation {
    fn solvable(&self, operators: &[Operator]) -> bool {
        solvable(
            self.operands[0],
            self.result,
            &self.operands[1..],
            &operators,
        )
    }
}

fn main() {
    let equations: Vec<_> = io::stdin()
        .lines()
        .map(|line| {
            line.expect("error reading input")
                .parse::<Equation>()
                .expect("error parsing equation")
        })
        .collect();

    println!(
        "part 1: {}",
        equations
            .iter()
            .filter(|eq| eq.solvable(&[add, mul]))
            .map(|eq| eq.result)
            .sum::<u64>()
    );
    println!(
        "part 2: {}",
        equations
            .iter()
            .filter(|eq| eq.solvable(&[add, mul, concat]))
            .map(|eq| eq.result)
            .sum::<u64>()
    );
}
