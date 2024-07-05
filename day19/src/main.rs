#![feature(generic_arg_infer)]
#![feature(int_roundings)]

use std::collections::HashMap;

use anyhow::{Context, Result};
mod parser;
use parser::{parse_rules, parse_xmases};
mod models;
use models::{Outcome, Permutation, RangeSet, Rule, Xmas};

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let part1 = part1(input)?;
    println!("part1: {part1}");

    let part2 = part2(input)?;
    println!("part2: {part2}");
    Ok(())
}

fn part1(s: &'static str) -> Result<isize> {
    let (rules, xmases) = s.split_once("\n\n").context("couldn't get both parts")?;
    let (_, rules) = parse_rules(rules).context("couldn't parse rules")?;
    let (_, xmases) = parse_xmases(xmases).context("couldn't parse rules")?;
    let rule_map: HashMap<String, &Rule> =
        rules.iter().map(|rule| (rule.name.clone(), rule)).collect();

    let mut total = 0;

    fn follow_outcome<'a>(
        outcome: &'a Outcome,
        xmas: &'a Xmas,
        rule_map: &'a HashMap<String, &'a Rule>,
    ) -> Result<&'a Outcome> {
        match outcome {
            Outcome::Target(name) => {
                let rule = rule_map
                    .get(name)
                    .context(format!("couldn't get target {}", name))?;
                follow_outcome(rule.apply(xmas), xmas, rule_map)
            }
            outcome => Ok(outcome),
        }
    }

    for xmas in xmases {
        let outcome = &Outcome::Target("in".into());
        match follow_outcome(outcome, &xmas, &rule_map)? {
            Outcome::Accepted => {
                total += xmas.total();
            }
            Outcome::Rejected => continue,
            _ => unreachable!(),
        }
    }

    Ok(total)
}

fn part2(s: &'static str) -> Result<isize> {
    let (rules, _xmases) = s.split_once("\n\n").context("couldn't get both parts")?;
    let (_, rules) = parse_rules(rules).context("couldn't parse rules")?;
    let rules_map = rules
        .into_iter()
        .map(|rule| (rule.name.clone(), rule))
        .collect::<HashMap<_, _>>();

    // start at in
    // follow each rule in the rules
    // track the state of conditions before entering
    // once you reach an A, multiply the conditions out
    let mut work = vec![("in", Permutation::default())];
    let mut count = 0;
    let mut accepted_permutations: Vec<Permutation> = vec![];

    while let Some((next_rule, permutation)) = work.pop() {
        let rule = rules_map.get(next_rule).context("couldn't get next node")?;

        let mut left_overs = permutation.clone(); // iterate through the each
                                                  // condition with less and less range

        for cond in &rule.conditions {
            let mut permutation = left_overs.clone();

            match cond {
                models::CondType::Unconditional(outcome) => match outcome {
                    Outcome::Target(target) => work.push((target, permutation)),
                    Outcome::Accepted => count += permutation.combos(),
                    Outcome::Rejected => continue,
                },
                models::CondType::Cond {
                    operator,
                    comparator,
                    field_name,
                    target,
                } => {
                    let (target_range, rest) = RangeSet::to_ranges(operator, comparator);

                    match target {
                        Outcome::Target(target) => {
                            left_overs = permutation.clone();
                            left_overs.add(field_name, rest);

                            permutation.add(field_name, target_range);
                            work.push((target.as_str(), permutation));
                        }
                        Outcome::Accepted => {
                            left_overs = permutation.clone();
                            left_overs.add(field_name, rest);

                            permutation.add(field_name, target_range);
                            count += permutation.combos();

                            accepted_permutations.push(permutation);
                        }
                        Outcome::Rejected => {
                            left_overs.add(field_name, rest);
                        }
                    }
                }
            }
        }
    }

    Ok(count)
}

#[test]
fn part2_works() {
    let input = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;
    assert_eq!(part2(input).unwrap(), 167409079868000)
}
