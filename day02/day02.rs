use std::io;

fn check(report: &[i32], i: usize, expected: i32, bad: usize, max_bad: usize) -> bool {
    if bad > max_bad {
        return false;
    }
    if i >= report.len() {
        return true;
    }
    if report[i] == expected {
        check(report, i + 1, expected + 1, bad, max_bad)
            || check(report, i + 1, expected + 2, bad, max_bad)
            || check(report, i + 1, expected + 3, bad, max_bad)
    } else {
        check(report, i + 1, expected, bad + 1, max_bad)
    }
}

fn safe(report: &[i32], max_bad: usize) -> bool {
    let reversed: Vec<_> = report.iter().rev().map(|x| *x).collect();

    check(report, 0, report[0], 0, max_bad)
        || check(&reversed, 0, reversed[0], 0, max_bad)
        || check(report, 1, report[1], 1, max_bad)
        || check(&reversed, 1, reversed[1], 1, max_bad)
}

fn main() {
    let reports: Vec<Vec<i32>> = io::stdin()
        .lines()
        .map(|line| {
            let line = line.expect("error reading input");
            line.split_whitespace()
                .map(|level| level.parse::<i32>().expect("error parsing level"))
                .collect()
        })
        .collect();

    println!(
        "part 1: {}",
        reports.iter().filter(|report| safe(report, 0)).count()
    );
    println!(
        "part 2: {}",
        reports.iter().filter(|report| safe(report, 1)).count()
    );
}
