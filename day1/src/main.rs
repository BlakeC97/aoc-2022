use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("data/input1.txt")
        .expect("Failed to load data/input1.txt");
    let reader = BufReader::new(input);

    let mut elf_calorie_counts: Vec<u64> = Vec::new();
    let mut curr_count = 0u64;
    for line in reader.lines().map_while(Result::ok) {
        // The parse will fail if we scan an empty line, which we'll use to delineate elves
        match line.parse::<u64>() {
            Ok(n) => {
                curr_count += n;
            }
            Err(_) => {
                elf_calorie_counts.push(curr_count);
                curr_count = 0;
            }
        }
    }

    // Sort in reverse order
    elf_calorie_counts.sort_by(|a, b| b.cmp(a));
    println!("{}", &elf_calorie_counts[..3].iter().sum::<u64>());
}
