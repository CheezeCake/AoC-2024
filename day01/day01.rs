use std::collections::HashMap;
use std::io;

fn main() {
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut right_count = HashMap::new();

    for line in io::stdin().lines() {
        let line = line.expect("error reading input");
        let (lstr, rstr) = line.split_once("   ").expect("error parsing input");
        let (l, r) = (
            lstr.parse::<u32>().expect("expected integer"),
            rstr.parse::<u32>().expect("expected integer"),
        );
        left.push(l);
        right.push(r);

        right_count
            .entry(r)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    left.sort();
    right.sort();

    println!(
        "part 1: {}",
        left.iter()
            .zip(right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum::<u32>()
    );
    println!(
        "part 2: {}",
        left.iter()
            .map(|l| *l * right_count.get(l).unwrap_or(&0))
            .sum::<u32>()
    );
}
