use anyhow::{Context, Error, Result};
use std::str::FromStr;

#[derive(Default)]
struct Frame {
    r: usize,
    g: usize,
    b: usize,
}

impl FromStr for Frame {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut frame: Frame = Default::default();
        for tuple in s.trim().split(", ") {
            let (num, color) = tuple.split_once(" ").context("can't split tuple")?;
            let num = num
                .parse::<usize>()
                .context(format!("couldn't get set num {}", s))?;
            match color {
                "red" => frame.r = num,
                "blue" => frame.b = num,
                "green" => frame.g = num,
                _ => return Err(anyhow::anyhow!("unknown color")),
            }
        }

        Ok(frame)
    }
}

impl Frame {
    fn is_possible(&self, r: usize, g: usize, b: usize) -> bool {
        r >= self.r && g >= self.g && b >= self.b
    }
}

struct Game {
    frames: Vec<Frame>,
    id: usize,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, frames) = s.trim().split_once(":").context("can't split game")?;
        let frames = frames
            .split("; ")
            .map(|frame| frame.parse::<Frame>())
            .collect::<Result<_>>()
            .context(format!("couldn't get frame from {}", s))?;

        let (_, id) = id.split_once(" ").context("can't split id")?;

        Ok(Self {
            frames,
            id: id
                .parse::<usize>()
                .context(format!("couldn't get id {}", id))?,
        })
    }
}

impl Game {
    fn is_possible(&self, r: usize, g: usize, b: usize) -> bool {
        self.frames.iter().all(|frame| frame.is_possible(r, g, b))
    }

    fn min_power_possible(&self) -> usize {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for frame in &self.frames {
            r = r.max(frame.r);
            g = g.max(frame.g);
            b = b.max(frame.b);
        }

        r * g * b
    }
}

fn parse(s: &str) -> Result<Vec<Game>> {
    s.trim().lines().map(|line| line.parse::<Game>()).collect()
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let games = parse(input)?;
    let part1 = games
        .iter()
        .filter(|g| g.is_possible(12, 13, 14))
        .map(|g| g.id)
        .sum::<usize>();
    println!("part1: {}", part1);

    let part2 = games
        .iter()
        .map(|g| g.min_power_possible())
        .sum::<usize>();
    println!("part2: {}", part2);
    Ok(())
}
