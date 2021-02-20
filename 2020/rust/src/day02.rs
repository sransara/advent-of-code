use lazy_static::lazy_static;
use regex::Regex;

struct Parsed<'a> {
    min: u32,
    max: u32,
    character: char,
    password: &'a str,
}

fn parse(line: &str) -> Parsed {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"([[:digit:]]+)-([[:digit:]]+) ([[:alpha:]]): ([[:alpha:]]+)").unwrap();
    }
    let caps = RE.captures(line).unwrap();
    Parsed {
        min: caps.get(1).unwrap().as_str().parse().unwrap(),
        max: caps.get(2).unwrap().as_str().parse().unwrap(),
        character: caps.get(3).unwrap().as_str().chars().next().unwrap(),
        password: caps.get(4).unwrap().as_str(),
    }
}

fn is_valid_pw(parsed: &Parsed) -> bool {
    let character_count =
        parsed.password.chars().fold(
            0,
            |acc, c| if c == parsed.character { acc + 1 } else { acc },
        );

    if parsed.min <= character_count && character_count <= parsed.max {
        true
    } else {
        false
    }
}

pub fn part1(input: String) {
    let lines_parsed: Vec<_> = input.lines().map(|line: &str| parse(line)).collect();
    let valid_pw_count =
        lines_parsed.iter().fold(
            0,
            |acc, parsed| if is_valid_pw(parsed) { acc + 1 } else { acc },
        );
    println!("{}", valid_pw_count);
}
