use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const LOWERCASE_PRIORITY_OFFSET: u64 = 'a' as u64 - 1;
const UPPERCASE_PRIORITY_OFFSET: u64 = 'A' as u64 - 1;

fn get_priority(c: char) -> u64 {
    if c.is_ascii_lowercase() {
        c as u64 - LOWERCASE_PRIORITY_OFFSET
    } else if c.is_ascii_uppercase() {
        c as u64 - UPPERCASE_PRIORITY_OFFSET + 26
    } else {
        c as u64
    }
}

fn main() {
    let input = File::open("data/input3.txt")
        .expect("Failed to load data/input3.txt");
    let reader = BufReader::new(input);

    let mut first_elf_seen_chars: HashSet<char> = HashSet::with_capacity(64);
    let mut second_elf_seen_chars: HashSet<char> = HashSet::with_capacity(64);
    let mut sum = 0;

    let mut line_iter = reader.lines();
    while let (Some(Ok(l1)), Some(Ok(l2)), Some(Ok(l3))) = (line_iter.next(), line_iter.next(), line_iter.next()) {
        first_elf_seen_chars.extend(l1.chars());
        second_elf_seen_chars.extend(l2.chars());

        for c in l3.chars() {
            if first_elf_seen_chars.contains(&c) && second_elf_seen_chars.contains(&c) {
                sum += get_priority(c);
                break;
            }
        }

        first_elf_seen_chars.clear();
        second_elf_seen_chars.clear();
    }

    println!("{sum}");
}


#[cfg(test)]
mod test {
    use crate::get_priority;

    #[test]
    fn test_get_priority() {
        assert_eq!(1, get_priority('a'));
        assert_eq!(26, get_priority('z'));
        assert_eq!(27, get_priority('A'));
        assert_eq!(52, get_priority('Z'));
    }
}
