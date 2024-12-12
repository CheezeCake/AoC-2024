use std::collections::HashMap;
use std::io;

type Stone = u64;

fn split_stone(stone: Stone) -> (Stone, Stone) {
    let digits = stone.ilog10() + 1;
    let x = 10_u64.pow(digits / 2);
    (stone / x, stone % x)
}

fn blink(stone: Stone) -> Vec<Stone> {
    match stone {
        0 => vec![1],
        _ => match (stone.ilog10() + 1) % 2 {
            0 => {
                let (first, second) = split_stone(stone);
                vec![first, second]
            }
            1 => vec![stone * 2024],
            _ => unreachable!(),
        },
    }
}

fn count_stones(stone: Stone, n: usize, mem: &mut HashMap<(Stone, usize), usize>) -> usize {
    if n == 0 {
        1
    } else {
        if let Some(&count) = mem.get(&(stone, n)) {
            return count;
        }

        let count = blink(stone).iter().map(|&stone| count_stones(stone, n - 1, mem)).sum();
        mem.insert((stone, n), count);
        count
    }
}

fn main() {
    let input = io::read_to_string(io::stdin()).expect("error reading input");
    let stones: Vec<_> = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("error parsing stone number"))
        .collect();

    let mut mem = HashMap::new();
    println!(
        "part 1: {}",
        stones
            .iter()
            .map(|&stone| count_stones(stone, 25, &mut mem))
            .sum::<usize>()
    );
    println!(
        "part 2: {}",
        stones
            .iter()
            .map(|&stone| count_stones(stone, 75, &mut mem))
            .sum::<usize>()
    );
}
