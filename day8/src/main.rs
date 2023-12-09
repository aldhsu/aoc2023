use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Context, Error, Result};

#[derive(Clone, Copy)]
enum Dir {
    L,
    R,
}

impl TryFrom<char> for Dir {
    type Error = Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            'L' => Dir::L,
            'R' => Dir::R,
            _ => return Err(anyhow!("unknown result direction: {}", value)),
        })
    }
}

#[derive(Debug)]
struct Tree<'a> {
    right: &'a str,
    left: &'a str,
}

impl<'a> Tree<'a> {
    fn branch(&'a self, dir: Dir) -> &'a str {
        match dir {
            Dir::L => self.left,
            Dir::R => self.right,
        }
    }
}

impl<'a> TryFrom<&'a str> for Tree<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> std::result::Result<Self, Self::Error> {
        let (left, right) = s
            .trim_matches('(')
            .trim_matches(')')
            .split_once(", ")
            .context("couldn't get tree branches")?;
        Ok(Self { left, right })
    }
}

#[derive(Debug)]
struct Map<'a> {
    map: HashMap<&'a str, Tree<'a>>,
}

impl<'a> TryFrom<&'a str> for Map<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> std::result::Result<Self, Self::Error> {
        // LFM = (PCJ, GQH)
        let map = s.trim()
            .lines()
            .map(|line| {
                let (name, tree) = line
                    .trim()
                    .split_once(" = ")
                    .context("couldn't get name and tree")?;
                let tree = Tree::try_from(tree)?;
                Ok((name, tree))
            })
            .collect::<Result<HashMap<_, _>>>()?;
        Ok(Self {
            map,
        })
    }
}

impl<'a> Map<'a> {
    fn next_dest(&'a self, current_pos: &'a str, dir: Dir) -> Option<&'a str> {
        self.map.get(current_pos).and_then(|t| Some(t.branch(dir)))
    }
}

fn part1(input: &str) -> Result<usize> {
    let (instructions, map) = input
        .split_once("\n\n")
        .context("couldn't get instructions")?;
    let instructions: Vec<Dir> = instructions
        .trim()
        .chars()
        .map(Dir::try_from)
        .collect::<Result<Vec<_>>>()
        .context("couldn't parse instructions")?;
    let mut instructions = instructions.into_iter().cycle();
    let map: Map = map.try_into()?;

    let mut current_pos = "AAA";
    let mut moves = 0;

    while current_pos != "ZZZ" {
        let Some(dir) = instructions.next() else { panic!("no instructions")};
        current_pos = map.next_dest(current_pos, dir).context(format!("couldn't find current pos {current_pos}"))?;
        moves += 1;
    }

    Ok(moves)
}

fn part2(input: &str) -> Result<usize> {
    let (instructions, map) = input
        .split_once("\n\n")
        .context("couldn't get instructions")?;
    let instructions: Vec<Dir> = instructions
        .trim()
        .chars() 
        .map(Dir::try_from)
        .collect::<Result<Vec<_>>>()
        .context("couldn't parse instructions")?;
    let instructions = instructions.into_iter().cycle();
    let map: Map = map.try_into()?;

    let mut current_positions = map.map.keys().filter(|key| key.ends_with('A')).cloned().collect::<Vec<_>>();
    let mut moves_needed = Vec::new();
    dbg!(&current_positions);

    for beginning_positions in &mut current_positions {
        let mut instructions = instructions.clone();
        let pos = beginning_positions;
        let mut moves = 0;
        while !pos.ends_with('Z') {
            let Some(dir) = instructions.next() else { panic!("no instructions")};
            *pos = map.next_dest(pos, dir).context(format!("couldn't find current pos {pos}"))?;
            moves += 1;
        }
        moves_needed.push(moves);
    }
    dbg!(&moves_needed);

    Ok(lcm(&moves_needed))
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input)?);
    println!("part2: {}", part2(input)?);
    Ok(())
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[test]
fn part2_works() {
let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
    assert_eq!(part2(input).unwrap(), 6);
}
