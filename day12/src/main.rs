use anyhow::{anyhow, Context, Error, Result};
use std::{collections::HashMap, str::FromStr};

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
enum Spring {
    Unknown,
    Working,
    Damaged,
}

impl TryInto<Spring> for char {
    type Error = Error;

    fn try_into(self) -> Result<Spring, Self::Error> {
        Ok(match self {
            '?' => Spring::Unknown,
            '.' => Spring::Working,
            '#' => Spring::Damaged,
            _ => return Err(anyhow!("unknown spring")),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Line<'a> {
    inner: &'a [Spring],
    pattern: &'a [usize],
}

impl<'a> From<&'a Value> for Line<'a> {
    fn from(value: &'a Value) -> Line<'a> {
        Line {
            inner: &value.inner[..],
            pattern: &value.pattern[..],
        }
    }
}

impl<'a> Line<'a> {
    fn arrangements(&self, cache: &mut HashMap<Self, usize>) -> usize {
        if let Some(count) = cache.get(self) {
            return *count;
        }

        fn pattern_fits(input: &[Spring], start: usize, pat_size: usize) -> bool {
            // must be the length because it is checking all items
            // all doesn't fail if it's too short
            if start + pat_size > input.len() {
                return false;
            }

            let fits = input
                .iter()
                .skip(start)
                .take(pat_size)
                .all(|spring| spring != &Spring::Working);

            if !fits {
                return false;
            }

            // have to check that next is not a broken otherwise too big
            !matches!(
                input.iter().skip(start + pat_size).next(),
                Some(Spring::Damaged)
            )
        }

        // if no patterns left we are finished, shouldn't ever hit the other case since it's
        // checked earlier
        let Some((len, pat_rest)) = self.pattern.split_first() else {
            if self.inner.len() > 0 && self.inner.iter().any(|spring| spring == &Spring::Damaged) {
                return 0;
            } else {
                return 1;
            }
        };
        // if patterns left and none left arrangement didn't work
        if self.inner.len() == 0 {
            return 0;
        }

        // must latch onto first #
        // doesn't have to latch first ?
        // go through find first ? or # #? = 0
        // go through find first # = #? = 0
        // this is the range it can iterate over
        let first_question_or_damaged = self
            .inner
            .iter()
            .position(|spring| spring != &Spring::Working)
            .unwrap_or(0);
        // or last quesiton mark
        let first_damaged_or_last_question = self
            .inner
            .iter()
            .position(|spring| spring == &Spring::Damaged)
            .unwrap_or_else(|| {
                self.inner
                    .iter()
                    .rposition(|spring| spring == &Spring::Unknown)
                    .unwrap_or(0)
            });

        (first_question_or_damaged..=first_damaged_or_last_question)
            .filter_map(|i| {
                // checked there is enough to fit the pattern in the remainder
                if pattern_fits(&self.inner, i, *len) {
                    // if we don't check for done now we will discard if it is done
                    // // why do we need to check this early?
                    // // because if there isn't any left than we can't give a new slice of inner
                    let next_inner = if i + len + 1 > self.inner.len() {
                        &[]
                    } else {
                        &self.inner[(i + len + 1)..]
                    };
                    let line = Line {
                        // + 1 because patterns can't be right next to each other
                        inner: next_inner,
                        pattern: pat_rest,
                    };
                    let count = line.arrangements(cache);
                    cache.insert(line, count);

                    Some(count)
                } else {
                    None
                }
            })
            .sum()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Value {
    inner: Vec<Spring>,
    pattern: Vec<usize>,
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (inner, pattern) = s.split_once(' ').context("couldn't get space")?;
        let inner = inner
            .chars()
            .map(|c| TryInto::<Spring>::try_into(c))
            .collect::<Result<Vec<_>>>()?;
        let pattern = pattern
            .split(',')
            .map(|str| str.parse::<usize>().context("couldn't parse pattern"))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { inner, pattern })
    }
}

impl Value {
    fn arrangements<'a>(&'a self, cache: &mut HashMap<Line<'a>, usize>) -> usize {
        Line::from(self).arrangements(cache)
    }

    fn expand(&mut self) {
        let inner_len = self.inner.len();
        let mut inner = Vec::new();
        std::mem::swap(&mut inner, &mut self.inner);
        let iter = inner
            .into_iter()
            .chain(std::iter::once(Spring::Unknown))
            .cycle();
        let new_inner = iter.take((inner_len + 1) * 5 - 1).collect();
        self.inner = new_inner;

        let inner_len = self.pattern.len();
        let mut pattern = Vec::new();
        std::mem::swap(&mut pattern, &mut self.pattern);
        let iter = pattern.into_iter().cycle();
        let new_pattern = iter.take(inner_len * 5).collect();
        self.pattern = new_pattern;
    }
}

#[test]
fn expand_test() {
    let mut val = Value {
        pattern: vec![1],
        inner: vec![Spring::Working, Spring::Damaged],
    };
    let expected_val = Value {
        pattern: vec![1, 1, 1, 1, 1],
        inner: vec![
            Spring::Working,
            Spring::Damaged,
            Spring::Unknown,
            Spring::Working,
            Spring::Damaged,
            Spring::Unknown,
            Spring::Working,
            Spring::Damaged,
            Spring::Unknown,
            Spring::Working,
            Spring::Damaged,
            Spring::Unknown,
            Spring::Working,
            Spring::Damaged,
        ],
    };
    val.expand();
    assert_eq!(val, expected_val)
}

fn part1(s: &str) -> Result<usize> {
    let mut cache = HashMap::new();
    let values = s
        .lines()
        .map(|line| line.parse::<Value>())
        .collect::<Result<Vec<_>>>()?;
    let mut total = 0;
    for line in values.iter() {
        total += line.arrangements(&mut cache)
    }
    Ok(total)
}

#[test]
fn part1_test() {
    let input = "???.# 1,1,1";
    assert_eq!(part1(input).unwrap(), 1);

    let input = "???.### 1,1,3";
    assert_eq!(part1(input).unwrap(), 1);

    let input = ".??..??...?##. 1,1,3";
    assert_eq!(part1(input).unwrap(), 4);

    //should eagerly latch onto present ones
    let input = "?? 1";
    assert_eq!(part1(input).unwrap(), 2);

    let input = "#? 1";
    assert_eq!(part1(input).unwrap(), 1);

    let input = "#?# 1,1";
    assert_eq!(part1(input).unwrap(), 1);

    let input = "?#? 1";
    assert_eq!(part1(input).unwrap(), 1);

    //should eagerly latch onto present ones
    let input = "?#?#?#?#?#?#?#? 1,3,1,6";
    assert_eq!(part1(input).unwrap(), 1);

    let input = "????.#...#... 4,1,1";
    assert_eq!(part1(input).unwrap(), 1);

    let input = "????.######..#####. 1,6,5";
    assert_eq!(part1(input).unwrap(), 4);

    let input = "?###???????? 3,2,1";
    assert_eq!(part1(input).unwrap(), 10);

    let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
    assert_eq!(part1(input).unwrap(), 21);

    let input = "#.# 1";
    assert_eq!(part1(input).unwrap(), 0);
}

fn part2(s: &str) -> Result<usize> {
    let mut cache = HashMap::new();
    let mut values = s
        .lines()
        .map(|line| line.parse::<Value>())
        .collect::<Result<Vec<_>>>()?;
    values.iter_mut().for_each(Value::expand);
    let mut total = 0;
    for line in values.iter() {
        total += line.arrangements(&mut cache);
    }
    Ok(total)
}

fn main() -> Result<(), Error> {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input)?);
    println!("part2: {}", part2(input)?);
    Ok(())
}
