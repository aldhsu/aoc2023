use std::{str::FromStr, array};

use anyhow::{anyhow, Context, Error, Result};

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("part1 {}", part1(input)?);
    println!("part2 {}", part2(input)?);
    Ok(())
}

struct Ins<'a> {
    seq: &'a str,
    op: Op,
    label: &'a str,
}

enum Op {
    Equal(u8),
    Minus,
}

impl<'a> TryFrom<&'a str> for Ins<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        let matcher = |c| c == '=' || c == '-';
        let mut splitter = value.split_inclusive(matcher);
        let label_op = splitter.next().context("can't get splitterop")?;

        let (label, _) = label_op.split_once(matcher).context("can't get label")?;

        let op = if label_op.ends_with('=') {
            let num = splitter.next().context("can't get num")?;
            Op::Equal(num.parse().context("can't parse num")?)
        } else if label_op.ends_with('-') {
            Op::Minus
        } else {
            return Err(anyhow!("couldn't get op {}", value));
        };

        Ok(Self {
            seq: value,
            op,
            label,
        })
    }
}

impl<'a> Ins<'a> {
    fn hash(&self) -> u8 {
        self.seq.chars().fold(0u8, |memo, c| {
            (memo.overflowing_add(c as u8).0).overflowing_mul(17).0
        })
    }

    fn address(&self) -> u8 {
        self.label.chars().fold(0u8, |memo, c| {
            (memo.overflowing_add(c as u8).0).overflowing_mul(17).0
        })
    }
}

#[derive(Debug)]
struct Lenses {
    boxes: [Vec<Lense>; 256],
}

impl Default for Lenses {
    fn default() -> Self {
        Self { boxes: array::from_fn(|_| Vec::default()) }
    }
}

impl Lenses {
    fn apply(&mut self, ins: Ins) {
        match ins.op {
            Op::Equal(focal_length) => {
                let lens_case = self
                    .boxes
                    .get_mut(ins.address() as usize)
                    .expect("couldn't get box");
                if let Some(lens) = lens_case.iter_mut().find(|lens| lens.label == ins.label) {
                    lens.focal_length = focal_length;
                } else {
                    lens_case.push(Lense {
                        label: ins.label.into(),
                        focal_length,
                    })
                }
            }
            Op::Minus => {
                let lens_case = self
                    .boxes
                    .get_mut(ins.address() as usize)
                    .expect("couldn't get box");
                if let Some(index) = lens_case.iter().position(|lens| lens.label == ins.label) {
                    lens_case.remove(index);
                }
            }
        }
    }

    fn power(&self) -> usize {
        self.boxes.iter().enumerate().flat_map(|(box_count, lenses)| {
            let box_factor = 1 + box_count;

            lenses.iter().enumerate().map(move |(lens_count, lense)| {
                let lens_count_factor = lens_count + 1;
                box_factor * lens_count_factor * lense.focal_length as usize
            })
        }).sum()
    }
}

#[derive(Default, Debug)]
struct Lense {
    label: String,
    focal_length: u8,
}

fn part1(s: &str) -> Result<usize> {
    let mut total = 0;
    for ins in s.trim().split(",") {
        let ins: Ins = ins.try_into()?;
        total += ins.hash() as usize;
    }

    Ok(total)
}

#[test]
fn part1_test() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";
    assert_eq!(part1(input).unwrap(), 1320);
}

fn part2(s: &str) -> Result<usize> {
    let mut lenses = Lenses::default();
    for ins in s.trim().split(",") {
        let ins: Ins = ins.try_into()?;
        lenses.apply(ins);
    }

    Ok(lenses.power())
}

#[test]
fn part2_test() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";
    assert_eq!(part2(input).unwrap(), 145);
}
