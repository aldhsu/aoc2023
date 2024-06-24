#![feature(generic_arg_infer)]
#![feature(int_roundings)]

use std::collections::HashMap;

use anyhow::{Result, Context};
mod parser;
use parser::{parse_rules, parse_xmases, Rule, Outcome, Xmas};

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let part1 = part1(input)?;
    println!("part1: {part1}");

    // let part2 = part2(input);
    // println!("part2: {part2}");
    Ok(())
}

fn part1(s: &'static str) -> Result<isize> {
    let (rules, xmases) = s.split_once("\n\n").context("couldn't get both parts")?;
    let (_, rules) = parse_rules(rules).context("couldn't parse rules")?;
    let (_, xmases) = parse_xmases(xmases).context("couldn't parse rules")?;
    let rule_map: HashMap<String, &Rule> = rules.iter().map(|rule| (rule.name.clone(), rule)).collect();
    dbg!(&rule_map.keys());

    let mut total = 0;

    fn follow_outcome<'a>(outcome: &'a Outcome, xmas: &'a Xmas, rule_map: &'a HashMap<String, &'a Rule>) -> Result<&'a Outcome> {
        match outcome {
            Outcome::Target(name) => {
                let rule = rule_map.get(name).context(format!("couldn't get target {}", name))?;
                follow_outcome(rule.apply(xmas), xmas, rule_map)
            },
            outcome => Ok(outcome)
        }
    }

    for xmas in xmases {
        let outcome = &Outcome::Target("in".into());
        match follow_outcome(outcome, &xmas, &rule_map)? {
            Outcome::Accepted => {
                total += xmas.total();
            },
            Outcome::Rejected => continue,
            _ => unreachable!()
        }
    }

    Ok(total)
}

// fn part2(s: &str) -> usize {
//     let (_, bps) = parse_rule(s).expect("couldn't parse");
//     todo!()
// }
