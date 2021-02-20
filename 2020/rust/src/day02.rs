use lazy_static::lazy_static;
use regex::Regex;

struct Parsed<'a> {
    n1: usize,
    n2: usize,
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
        n1: caps.get(1).unwrap().as_str().parse().unwrap(),
        n2: caps.get(2).unwrap().as_str().parse().unwrap(),
        character: caps.get(3).unwrap().as_str().chars().next().unwrap(),
        password: caps.get(4).unwrap().as_str(),
    }
}

fn old_is_valid_pw(parsed: &Parsed) -> bool {
    let character_count =
        parsed.password.chars().fold(
            0,
            |acc, c| if c == parsed.character { acc + 1 } else { acc },
        );

    if parsed.n1 <= character_count && character_count <= parsed.n2 {
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
            |acc, parsed| if old_is_valid_pw(parsed) { acc + 1 } else { acc },
        );
    println!("{}", valid_pw_count);
}

fn is_valid_pw(parsed: &Parsed) -> bool {
    let first_char = parsed.password.chars().nth(parsed.n1 - 1);
    let second_char = parsed.password.chars().nth(parsed.n2 - 1);
    
    match (first_char, second_char) {
        (Some(x), Some(y)) if x == parsed.character && y != parsed.character => true,
        (Some(x), None) if x == parsed.character => true,
        (Some(x), Some(y)) if x != parsed.character && y == parsed.character => true,
        (None, Some(y)) if y == parsed.character => true,
        _ => false,
    }
}

pub fn part2(input: String) {
    let lines_parsed: Vec<_> = input.lines().map(|line: &str| parse(line)).collect();
    let valid_pw_count =
        lines_parsed.iter().fold(
            0,
            |acc, parsed| if is_valid_pw(parsed) { acc + 1 } else { acc },
        );
    println!("{}", valid_pw_count);
}