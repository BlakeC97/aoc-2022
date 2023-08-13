use std::path::PathBuf;
use anyhow::Result;
use camino::Utf8PathBuf;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, multispace0, not_line_ending, space1, u64};
use nom::combinator::map;
use nom::IResult;
use nom::sequence::{pair, preceded, separated_pair, terminated};

#[derive(Debug)]
pub enum Cmd {
    Cd { dir: Utf8PathBuf },
    Ls,
}

#[derive(Debug)]
pub enum Entry {
    Dir { name: Utf8PathBuf },
    File { name: Utf8PathBuf, size: u64 },
}

#[derive(Debug)]
pub enum Line {
    Cmd(Cmd),
    Entry(Entry),
}

fn parse_command(input: &str) -> IResult<&str, Line> {
    // Skip past the `$ ` part
    let (input, _) = pair(tag("$"), space1)(input)?;

    let ls_parser = map(terminated(tag("ls"), line_ending), |_| Cmd::Ls);
    let cd_parser = map(preceded(pair(tag("cd"), space1), not_line_ending), |s: &str| Cmd::Cd {dir: s.into()});

    let (input, cmd) = alt((ls_parser, cd_parser))(input)?;

    // The ls parser can leave a space behind (cd will not), so trim that off
    let (input, _) = multispace0(input)?;

    Ok((input, Line::Cmd(cmd)))
}

fn parse_entry(input: &str) -> IResult<&str, Line> {
    let dir_parser = map(preceded(pair(tag("dir"), space1), not_line_ending), |name: &str| Entry::Dir {name: name.into()});
    let file_parser = map(separated_pair(u64, space1, not_line_ending), |(size, name): (u64, &str)| Entry::File {name: name.into(), size});

    let (input, entry) = alt((dir_parser, file_parser))(input)?;

    // Thanks to `not_line_ending`, we'll have a line ending in the output -- trim it
    let (input, _) = multispace0(input)?;

    Ok((input, Line::Entry(entry)))
}

fn main() -> Result<()> {
    let input = include_str!("../data/input.txt");

    let mut input_parser = alt((parse_command, parse_entry));
    while let Ok((input, line)) = input_parser(input) {
        println!("{line:?}");
    }

    Ok(())
}
