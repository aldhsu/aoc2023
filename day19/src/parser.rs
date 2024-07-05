use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::sequence::tuple;
use nom::IResult;

use crate::models::{Rule, CondType, Outcome, Xmas, Operator, FieldName};

#[test]
fn parsing_rule_works() {
    let input = "px{a<2006:qkq,m>2090:A,rfg}";
    let (_, rule) = parse_rule(input).unwrap();
    let xmas = Xmas {
        a: 2005,
        ..Xmas::default()
    };

    assert_eq!(rule.apply(&xmas), &Outcome::Target("qkq".into()));

    let xmas = Xmas {
        a: 2006,
        m: 2091,
        ..Xmas::default()
    };

    assert_eq!(rule.apply(&xmas), &Outcome::Accepted);

    let xmas = Xmas {
        a: 2006,
        m: 2090,
        ..Xmas::default()
    };

    assert_eq!(rule.apply(&xmas), &Outcome::Target("rfg".into()));
}

pub fn parse_xmases(s: &str) -> IResult<&str, Vec<Xmas>> {
    separated_list1(tag("\n"), parse_xmas)(s)
}

#[test]
fn can_parse_xmas() {
    let input = r#"{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;
    let (_, xmases) = parse_xmases(input).unwrap();
    assert_eq!(xmases.len(), 5)
}

fn parse_xmas(s: &str) -> IResult<&str, Xmas> {
    // {x=787,m=2655,a=1222,s=2876}
    let (s, key_values) = delimited(
        tag("{"),
        separated_list1(tag(","), parse_key_value),
        tag("}"),
    )(s)?;
    let mut xmas = Xmas::default();

    for (key, value) in key_values {
        let value = value.parse::<isize>().unwrap();

        match key {
            "x" => xmas.x = value,
            "m" => xmas.m = value,
            "a" => xmas.a = value,
            "s" => xmas.s = value,
            _ => unreachable!(),
        }
    }

    Ok((s, xmas))
}

fn parse_key_value(s: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, tag("="), digit1)(s)
}

pub fn parse_rules(s: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(tag("\n"), parse_rule)(s)
}
#[test]
fn parse_rules_works() {
    let input = r#"bcj{s>2236:A,s<1879:A,x<3217:R,R}
dx{s>2913:fjr,m>3556:A,a>845:A,qmf}"#;
    let (_, rules) = parse_rules(input).unwrap();
    assert_eq!(rules.len(), 2)
}

fn parse_rule(s: &str) -> IResult<&str, Rule> {
    let (s, (name, conds)) = tuple((alpha1, delimited(tag("{"), parse_conds, tag("}"))))(s)?;

    let rule = Rule {
        conditions: conds,
        name: name.into(),
    };

    Ok((s, rule))
}

fn parse_conds(s: &str) -> IResult<&str, Vec<CondType>> {
    separated_list1(tag(","), parse_cond)(s)
}
#[test]
fn parse_conds_works() {
    let (_, conds) = parse_conds("rfg,rfg").unwrap();
    assert_eq!(conds.len(), 2)
}

fn parse_cond(s: &str) -> IResult<&str, CondType> {
    // a<2006:qkq
    // m>2090:A
    // rfg
    alt((parse_full_cond, parse_no_cond))(s)
}

fn parse_outcome(s: &str) -> IResult<&str, Outcome> {
    // rfg
    let (s, outcome) = alpha1(s)?;
    let outcome = match outcome {
        "A" => Outcome::Accepted,
        "R" => Outcome::Rejected,
        val => Outcome::Target(val.into()),
    };
    Ok((s, outcome))
}
#[test]
fn parse_outcome_works() {
    assert_eq!(
        parse_outcome("rfg").unwrap(),
        ("", Outcome::Target("rfg".into()))
    )
}

fn parse_no_cond(s: &str) -> IResult<&str, CondType> {
    let (s, outcome) = parse_outcome(s)?;

    Ok((
        s,
        CondType::Unconditional(outcome),
    ))
}
#[test]
fn parse_no_cond_works() {
    let (_, cond) = parse_no_cond("rfg").unwrap();
    let xmas: Xmas = Default::default();
    assert_eq!(cond.apply(&xmas), Some(&Outcome::Target("rfg".into())))
}

fn parse_full_cond(s: &str) -> IResult<&str, CondType> {
    // a<2006:qkq
    // m>2090:A
    let (s, (field, operator, comparator, _, target)) = tuple((
        alpha1,
        alt((tag("<"), tag(">"))),
        digit1,
        tag(":"),
        parse_outcome,
    ))(s)?;

    let comparator = comparator.parse::<isize>().unwrap();
    let field_name = match field {
        "x" => FieldName::X,
        "m" => FieldName::M,
        "a" => FieldName::A,
        "s" => FieldName::S,
        _ => unreachable!(),
    };

    let operator = match operator {
        ">" => Operator::Greater,
        "<" => Operator::Lesser,
        _ => unreachable!(),
    };

    let cond = CondType::Cond{
        target,
        operator,
        comparator,
        field_name,
    };

    Ok((s, cond))
}

#[test]
fn parse_full_cond_works() {
    let (_, cond) = parse_full_cond("a<2006:qkq").unwrap();
    let xmas: Xmas = Xmas {
        a: 2005,
        ..Default::default()
    };
    assert_eq!(cond.apply(&xmas), Some(&Outcome::Target("qkq".into())));

    let xmas: Xmas = Xmas {
        a: 2007,
        ..Default::default()
    };
    assert_eq!(cond.apply(&xmas), None)
}
