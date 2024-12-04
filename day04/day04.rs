use std::io;

fn find_word(
    word: &str,
    grid: &[Vec<char>],
    line: usize,
    col: usize,
    direction: &(isize, isize),
) -> bool {
    for (i, c) in word.chars().enumerate() {
        let nline = line as isize + direction.0 * (i as isize);
        let ncol = col as isize + direction.1 * (i as isize);
        if nline < 0
            || (nline as usize) >= grid.len()
            || ncol < 0
            || (ncol as usize) >= grid[nline as usize].len()
            || grid[nline as usize][ncol as usize] != c
        {
            return false;
        }
    }

    true
}

fn is_x_mass(a: char, b: char) -> bool {
    (a == 'M' || a == 'S') && (b == 'M' || b == 'S') && a != b
}

fn main() {
    let grid: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.expect("error reading input").chars().collect())
        .collect();

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            for dir in directions {
                if find_word("XMAS", &grid, i, j, &dir) {
                    count += 1;
                }
            }
        }
    }
    println!("part 1: {}", count);

    count = 0;
    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            if grid[i][j] == 'A'
                && is_x_mass(grid[i - 1][j - 1], grid[i + 1][j + 1])
                && is_x_mass(grid[i + 1][j - 1], grid[i - 1][j + 1])
            {
                count += 1;
            }
        }
    }
    println!("part 2: {}", count);
}
