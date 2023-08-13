use std::cell::RefCell;
use std::collections::VecDeque;
use anyhow::Result;
use nom::branch::alt;
use nom::character::complete::{char, digit1, space0};
use nom::combinator::{map};
use nom::bytes::complete::{tag, take_until};
use nom::bytes::complete::take;
use nom::IResult;
use nom::multi::{separated_list0};
use nom::sequence::{delimited, terminated};
use std::fmt;
use std::fmt::Formatter;

pub struct Crate(char);

impl fmt::Debug for Crate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

#[derive(Debug)]
pub struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_crate(input: &str) -> IResult<&str, Crate> {
    let f = delimited(char('['), take(1_usize), char(']'));
    map(f, |s: &str| Crate(s.chars().next().unwrap()))(input)
}

fn parse_hole(input: &str) -> IResult<&str, ()> {
    map(tag("   "), |_| ())(input)
}

fn parse_crate_or_hole(input: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Option<Crate>>> {
    // We want precisely one space in-between, any variant of space{0,1} will not capture the structure
    separated_list0(tag(" "), parse_crate_or_hole)(input)
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<Option<Crate>>>> {
    separated_list0(tag("\n"), parse_line)(input)
}

fn parse_to_instructions(input: &str) -> IResult<&str, &str> {
    take_until("move")(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let parse_usize = |c: &str| c.parse::<usize>().unwrap_or(0);

    let (input, amount) = delimited(terminated(tag("move"), space0), map(digit1, parse_usize), space0)(input)?;
    let (input, from) = delimited(terminated(tag("from"), space0), map(digit1, parse_usize), space0)(input)?;
    let (input, to) = delimited(terminated(tag("to"), space0), map(digit1, parse_usize), space0)(input)?;

    // Positions in input are 1-based, so convert to 0-based here for ease-of-use
    Ok((input, Instruction {
        amount,
        from: from - 1,
        to: to - 1,
    }))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list0(tag("\n"), parse_instruction)(input)
}

fn transpose_and_flip<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    debug_assert!(!v.is_empty());

    let mut transposed: Vec<Vec<T>> = (0..v[0].len()).map(|_| vec![]).collect();

    for original_row in v {
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            if let Some(safe_item) = item {
                transposed_row.insert(0, safe_item);
            }
        }
    }

    transposed
}

fn main() -> Result<()> {
    let input = include_str!("../data/input.txt");

    // Step 1a: Get the crates
    let (input, mut crates) = parse_lines(input)?;
    // Hacky workaround: The last row is always blank, so just pop it from the vecs
    if let Some(last_vec) = crates.last() {
        if last_vec.is_empty() { crates.pop(); }
    }
    // Step 1b: Remove the `None` entries + transpose the vectors for ease of push/pop
    // Need to use RefCell here since we need to borrow 2 Vecs at the same time, which isn't possible
    // under normal Rust rules
    let crates: Vec<_> = transpose_and_flip(crates).into_iter().map(RefCell::new).collect();

    // Step 2: Skip until the instructions ("move X from Y to Z")
    let (input, _) = parse_to_instructions(input)?;

    // Step 3: Parse the instructions and move the crates
    let (_, instructions) = parse_instructions(input)?;

    let mut carried_by_crane = VecDeque::new();
    for instruction in &instructions {
        for _ in 0..instruction.amount {
            let mut src = crates.get(instruction.from).unwrap().borrow_mut();
            carried_by_crane.push_back(src.pop().unwrap());
        }

        let mut dest = crates.get(instruction.to).unwrap().borrow_mut();
        while !carried_by_crane.is_empty() {
            dest.push(carried_by_crane.pop_back().unwrap());
        }
    }

    for row in &crates {
        print!("{}", row.borrow().last().unwrap().0);
    }

    Ok(())
}
