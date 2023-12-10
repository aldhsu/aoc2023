use anyhow::{Context, Result};

fn part1(input: &str) -> Result<i32> {
    let lines = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| {
                    num.parse::<i32>()
                        .context(format!("can't parse num {}", num))
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    fn predict_next(slice: &[i32]) -> i32 {
        let differences = slice
            .windows(2)
            .map(|win| win[1] - win[0])
            .collect::<Vec<_>>();

        if let Some(a) = differences.last() {
            if differences.iter().any(|d| d != &0) {
                return a + predict_next(&differences);
            } else {
                return 0;
            }
        } else {
            0
        }
    }

    let mut total = 0;

    for line in lines {
        total += line.last().unwrap() + predict_next(&line);
    }

    Ok(total)
}

fn part2(input: &str) -> Result<i32> {
    let lines = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| {
                    num.parse::<i32>()
                        .context(format!("can't parse num {}", num))
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    fn predict_previous(slice: &[i32]) -> i32 {
        let differences = slice
            .windows(2)
            .map(|win| win[1] - win[0])
            .collect::<Vec<_>>();

        if let Some(a) = differences.first() {
            if differences.iter().any(|d| d != &0) {
                return a - predict_previous(&differences);
            } else {
                return 0;
            }
        } else {
            0
        }
    }

    let mut total = 0;

    for line in lines {
        total += line.first().unwrap() - predict_previous(&line);
    }

    Ok(total)
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let part1 = part1(input)?;
    println!("part1: {part1}");
    let part2 = part2(input)?;
    println!("part2: {part2}");

    Ok(())
}

#[test]
fn part1_works() {
    let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
    assert_eq!(part1(input).unwrap(), 114)
}

#[test]
fn part2_works() {
    let input = r#"10 13 16 21 30 45"#;
    assert_eq!(part2(input).unwrap(), 5)
}
