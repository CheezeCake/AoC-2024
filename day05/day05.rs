use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

fn correct_order(update: &[usize], dependencies: &HashMap<usize, HashSet<usize>>) -> Vec<usize> {
    let update_set: HashSet<usize> = update.iter().cloned().collect();
    let mut print_queue: VecDeque<usize> = update.iter().cloned().collect();
    let mut printed: HashSet<usize> = HashSet::new();
    let mut print_order = Vec::new();
    let mut dependencies = dependencies.clone();

    while let Some(page) = print_queue.pop_front() {
        if dependencies
            .get(&page)
            .unwrap_or(&HashSet::new())
            .intersection(&update_set)
            .count()
            == 0
        {
            print_order.push(page);
            printed.insert(page);
            for (_, deps) in &mut dependencies {
                deps.remove(&page);
            }
        } else {
            print_queue.push_back(page);
        }
    }

    print_order
}

fn middle_page(update: &[usize]) -> usize {
    update[update.len() / 2]
}

fn main() {
    let input = io::read_to_string(io::stdin()).expect("error reading input");
    let (ordering_rules, updates) = input.split_once("\n\n").expect("error parsing input");
    let ordering_rules: Vec<(usize, usize)> = ordering_rules
        .lines()
        .map(|line| line.split_once('|').expect("error parsing ordering rule"))
        .map(|(x, y)| {
            (
                x.parse::<usize>().expect("error parsing page number"),
                y.parse::<usize>().expect("error parsing page number"),
            )
        })
        .collect();
    let updates: Vec<Vec<usize>> = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|page| page.parse::<usize>().expect("erorr parsing page number"))
                .collect()
        })
        .collect();

    let mut dependencies: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (x, y) in ordering_rules {
        dependencies.entry(y).or_insert(HashSet::new()).insert(x);
    }

    let correctly_ordered_updates: Vec<_> = updates
        .iter()
        .map(|update| correct_order(update, &dependencies))
        .collect();

    println!(
        "part 1: {}",
        updates
            .iter()
            .zip(correctly_ordered_updates.iter())
            .filter(|(update, correctly_ordered_update)| update == correctly_ordered_update)
            .map(|(update, _)| middle_page(update))
            .sum::<usize>()
    );
    println!(
        "part 2: {}",
        updates
            .iter()
            .zip(correctly_ordered_updates.iter())
            .filter(|(update, correctly_ordered_update)| update != correctly_ordered_update)
            .map(|(_, correctly_ordered_update)| middle_page(correctly_ordered_update))
            .sum::<usize>()
    );
}
