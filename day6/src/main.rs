use std::collections::BTreeSet;
use itertools::Itertools;

const WINDOW_SIZE: usize = 14;

fn main() {
    // Assuming it's ASCII (which, for AoC, it is)
    let input = include_str!("../data/input.txt").as_bytes();

    let result = input
        .windows(WINDOW_SIZE)
        .enumerate()
        .find_or_first(|&(_idx, window)| BTreeSet::from_iter(window).len() == WINDOW_SIZE)
        .expect("There must be _some_ solution in the input!");

    println!("Position: {}\nChar array: {:?}", result.0 + WINDOW_SIZE, result.1);
}
