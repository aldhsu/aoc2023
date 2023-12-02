fn get_first(line: &str) -> char {
    line.chars()
        .find(|c| c.is_ascii_digit())
        .expect("couldn't find first")
}

fn get_last(line: &str) -> char {
    line.chars()
        .rev()
        .find(|c| c.is_ascii_digit())
        .expect("couldn't find last")
}

fn part1(s: &str) -> u64 {
    s.trim()
        .split("\n")
        .map(|line| {
            let first = get_first(line);
            let last = get_last(line);

            format!("{}{}", first, last)
                .parse::<u64>()
                .expect("couldn't get value")
        })
        .sum()
}

const NUMBER_STRINGS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn starts_with_string(s: &str) -> Option<usize> {
    NUMBER_STRINGS
        .into_iter()
        .position(|num| s.starts_with(num))
}

fn ends_with_string(s: &str) -> Option<usize> {
    NUMBER_STRINGS.into_iter().position(|num| s.ends_with(num))
}

fn find_first<'a>(line: &'a str) -> String {
    if line.starts_with(|c: char| c.is_ascii_digit()) {
        line[0..1].into()
    } else if let Some(pos) = starts_with_string(line) {
        format!("{}", pos + 1)
    } else {
        find_first(&line[1..])
    }
}

fn find_last<'a>(line: &'a str) -> String {
    dbg!(&line);
    if line.ends_with(|c: char| c.is_ascii_digit()) {
        line.chars().last().unwrap().into()
    } else if let Some(pos) = ends_with_string(line) {
        format!("{}", pos + 1)
    } else {
        let len = match line.len().checked_sub(1) {
            Some(l) => l,
            None => panic!("couldn't get for line {}", line),
        };
        find_last(&line[..len])
    }
}

fn part2(s: &str) -> u64 {
    s.trim()
        .split("\n")
        .map(|line| {
            let first = find_first(line);
            let last = find_last(line);

            format!("{}{}", first, last)
                .parse::<u64>()
                .expect("couldn't get value")
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}
