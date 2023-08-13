use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;
use crate::IntervalParseError::BadInputError;

#[derive(Debug)]
pub enum IntervalParseError {
    BadInputError,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for IntervalParseError {
    fn from(error: ParseIntError) -> Self {
        IntervalParseError::ParseIntError(error)
    }
}

#[derive(Debug, PartialEq)]
pub struct Interval {
    start: u64,
    end: u64,
}

impl Interval {
    pub fn new(start: u64, end: u64) -> Interval {
        Interval {start, end}
    }

    pub fn length(&self) -> u64 {
        self.end - self.start + 1
    }

    pub fn contains(&self, other: &Interval) -> bool {
        let (longer, shorter) = if self.length() >= other.length() {
            (self, other)
        } else {
            (other, self)
        };

        match shorter.start >= longer.start {
            true => shorter.end <= longer.end,
            false => false,
        }
    }

    pub fn overlaps(&self, other: &Interval) -> bool {
        let (longer, shorter) = if self.length() >= other.length() {
            (self, other)
        } else {
            (other, self)
        };

        longer.start <= shorter.end && longer.end >= shorter.start
    }
}

impl FromStr for Interval {
    type Err = IntervalParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        if let (Some(start), Some(end)) = (split.next(), split.next()) {
            Ok(Interval::new(start.parse()?, end.parse()?))
        } else {
            Err(BadInputError)
        }
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

fn main() -> Result<(), IntervalParseError> {
    let input = File::open("data/input.txt")
        .expect("Failed to load data/input.txt");
    let reader = BufReader::new(input);

    let mut contained_pairs = 0u64;
    let mut overlapping_pairs = 0u64;
    for line in reader.lines().map_while(Result::ok) {
        let mut line_split = line.split(',');
        if let (Some(i1), Some(i2)) = (line_split.next(), line_split.next()) {
            let i1: Interval = i1.parse()?;
            let i2: Interval = i2.parse()?;

            if i1.contains(&i2) {
                contained_pairs += 1;
                overlapping_pairs += 1;
            } else if i1.overlaps(&i2) {
                overlapping_pairs += 1;
            }
        }
    }

    println!("Contained pairs: {contained_pairs}\nOverlapping pairs: {overlapping_pairs}");

    Ok(())
}


#[cfg(test)]
mod test {
    use crate::Interval;

    #[test]
    fn test_interval_parse() {
        assert_eq!(Interval::new(0, 0), "0-0".parse().unwrap());
        assert_eq!(Interval::new(1, 3), "1-3".parse().unwrap());
    }

    #[test]
    fn test_contains() {
        let i1 = Interval::new(2, 4);
        let i2 = Interval::new(6, 8);
        assert!(!i1.contains(&i2));

        let i1 = Interval::new(2, 3);
        let i2 = Interval::new(4, 5);
        assert!(!i1.contains(&i2));

        let i1 = Interval::new(5, 7);
        let i2 = Interval::new(7, 9);
        assert!(!i1.contains(&i2));

        let i1 = Interval::new(2, 8);
        let i2 = Interval::new(3, 7);
        assert!(i1.contains(&i2));

        let i1 = Interval::new(6, 6);
        let i2 = Interval::new(4, 6);
        assert!(i1.contains(&i2));

        let i1 = Interval::new(2, 6);
        let i2 = Interval::new(4, 8);
        assert!(!i1.contains(&i2));
    }

    #[test]
    fn test_overlaps() {
        let i1 = Interval::new(2, 4);
        let i2 = Interval::new(6, 8);
        assert!(!i1.overlaps(&i2));

        let i1 = Interval::new(2, 3);
        let i2 = Interval::new(4, 5);
        assert!(!i1.overlaps(&i2));

        let i1 = Interval::new(5, 7);
        let i2 = Interval::new(7, 9);
        assert!(i1.overlaps(&i2));

        let i1 = Interval::new(2, 8);
        let i2 = Interval::new(3, 7);
        assert!(i1.overlaps(&i2));

        let i1 = Interval::new(6, 6);
        let i2 = Interval::new(4, 6);
        assert!(i1.overlaps(&i2));

        let i1 = Interval::new(2, 6);
        let i2 = Interval::new(4, 8);
        assert!(i1.overlaps(&i2));
    }
}