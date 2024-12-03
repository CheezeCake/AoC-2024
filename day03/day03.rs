use std::io;

enum PatternNode {
    Str(String),
    Int,
}

fn match_end(s: &str, spos: usize, p: &[PatternNode], ppos: usize) -> Option<usize> {
    if ppos >= p.len() {
        return Some(spos);
    }
    if spos >= s.len() {
        return None;
    }

    match &p[ppos] {
        PatternNode::Str(str) => {
            if s[spos..].starts_with(str) {
                match_end(s, spos + str.len(), p, ppos + 1)
            } else {
                None
            }
        }
        PatternNode::Int => {
            if s.as_bytes()[spos].is_ascii_digit() {
                let x = match_end(s, spos + 1, p, ppos + 1);
                if x.is_some() {
                    x
                } else {
                    match_end(s, spos + 1, p, ppos)
                }
            } else {
                None
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Match {
    start: usize,
    end: usize,
}

fn find_matches(s: &str, p: &[PatternNode]) -> Vec<Match> {
    (0..s.len())
        .map(|start| (start, match_end(s, start, p, 0)))
        .filter(|(_, end)| end.is_some())
        .map(|(start, end)| Match {
            start,
            end: end.unwrap(),
        })
        .collect()
}

fn parse_and_compute_mul(s: &str, mul: &Match) -> u32 {
    let (x, y) = s[mul.start + 4..mul.end - 1].split_once(",").unwrap();
    x.parse::<u32>().unwrap() * y.parse::<u32>().unwrap()
}

fn main() {
    let memory: Vec<String> = io::stdin()
        .lines()
        .map(|line| line.expect("error reading input"))
        .collect();

    let mul_pattern = [
        PatternNode::Str(String::from("mul(")),
        PatternNode::Int,
        PatternNode::Str(String::from(",")),
        PatternNode::Int,
        PatternNode::Str(String::from(")")),
    ];

    let mut part1: u32 = 0;
    let mut part2: u32 = 0;
    let mut enabled = true;

    for s in memory {
        let mut multiplications = find_matches(&s, &mul_pattern);
        part1 += multiplications
            .iter()
            .map(|m| parse_and_compute_mul(&s, m))
            .sum::<u32>();

        let mut dos = find_matches(&s, &[PatternNode::Str(String::from("do()"))]);
        let mut donts = find_matches(&s, &[PatternNode::Str(String::from("don't()"))]);

        let mut instructions = Vec::new();
        instructions.append(&mut multiplications);
        instructions.append(&mut dos);
        instructions.append(&mut donts);
        instructions.sort_by_key(|m| m.start);

        for inst in instructions {
            let inst_str = &s[inst.start..inst.end];
            if inst_str.starts_with("mul") && enabled {
                part2 += parse_and_compute_mul(&s, &inst);
            } else if inst_str.starts_with("do()") {
                enabled = true;
            } else {
                enabled = false;
            }
        }
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
