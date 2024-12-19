use std::collections::HashMap;
use std::io;

fn possible_arrangements<'a>(
    design: &'a str,
    available_patterns: &[&str],
    mem: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.len() == 0 {
        1
    } else if let Some(&n) = mem.get(design) {
        n
    } else {
        let n: usize = available_patterns
            .iter()
            .filter(|&pattern| design.starts_with(pattern))
            .map(|pattern| possible_arrangements(&design[pattern.len()..], available_patterns, mem))
            .sum();
        mem.insert(design, n);
        n
    }
}

fn main() {
    let input = io::read_to_string(io::stdin()).expect("error reading input");
    let (available_patterns, designs) = input.split_once("\n\n").expect("error parsing input");
    let available_patterns: Vec<_> = available_patterns.split(", ").collect();
    let designs: Vec<_> = designs.lines().collect();

    let designs_arrangements: Vec<_> = designs
        .iter()
        .map(|design| possible_arrangements(design, &available_patterns, &mut HashMap::new()))
        .collect();

    println!(
        "part 1: {}",
        designs_arrangements
            .iter()
            .filter(|&&arrangements| arrangements > 0)
            .count()
    );
    println!("part 2: {}", designs_arrangements.iter().sum::<usize>());
}
