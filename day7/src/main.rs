use anyhow::{anyhow, Context, Error, Result};
use std::{collections::HashMap, str::FromStr, marker::PhantomData};


#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Hash)]
struct NoJoke;
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Hash)]
struct Jokes;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Hash)]
enum Card<T> {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Unused(PhantomData<T>),
}

impl TryFrom<char> for Card<NoJoke> {
    type Error = Error;

    fn try_from(s: char) -> std::result::Result<Self, Self::Error> {
        Ok(match s {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => return Err(anyhow!("couldn't parse card: {}", s)),
        })
    }
}

impl TryFrom<char> for Card<Jokes> {
    type Error = Error;

    fn try_from(s: char) -> std::result::Result<Self, Self::Error> {
        Ok(match s {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Joker,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => return Err(anyhow!("couldn't parse card: {}", s)),
        })
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum HandType<T> {
    HighCard([Card<T>; 5]),
    Pair([Card<T>; 5]),         // high low kickers AA B C D E
    TwoPair([Card<T>; 5]),      // high low kickers AA BB D
    ThreeOfAKind([Card<T>; 5]), // AAA B C
    FullHouse([Card<T>; 5]),    // AAA BB
    FourOfAKind([Card<T>; 5]),  // AAAA B
    FiveOfAKind([Card<T>; 5]),
}

impl From<[Card<NoJoke>; 5]> for HandType<NoJoke> {
    fn from(cards: [Card<NoJoke>; 5]) -> Self {
        let mut count: HashMap<&Card<NoJoke>, usize> = Default::default();
        for card in &cards {
            *count.entry(card).or_insert(0) += 1;
        }

        let mut vals = count.into_iter().collect::<Vec<(&Card<NoJoke>, usize)>>();
        vals.sort_by_key(|(card, count)| (*count, *card));
        vals.reverse();

        match vals[..] {
            [(a, 5)] => HandType::FiveOfAKind(cards),
            [(a, 4), (b, 1)] => HandType::FourOfAKind(cards),
            [(a, 3), (b, 2)] => HandType::FullHouse(cards),
            [(a, 3), (b, 1), (c, 1)] => HandType::ThreeOfAKind(cards),
            [(a, 2), (b, 2), (c, 1)] => HandType::TwoPair(cards),
            [(a, 2), (b, 1), (c, 1), (d, 1)] => HandType::Pair(cards),
            [(a, 1), (b, 1), (c, 1), (d, 1), (e, 1)] => HandType::HighCard(cards),
            _ => {
                unreachable!()
            }
        }
    }
}

impl From<[Card<Jokes>; 5]> for HandType<Jokes> {
    fn from(cards: [Card<Jokes>; 5]) -> Self {
        let mut count: HashMap<&Card<Jokes>, usize> = Default::default();
        for card in &cards {
            *count.entry(card).or_insert(0) += 1;
        }

        let jokers = count.remove(&Card::Joker);

        let mut vals = count.into_iter().collect::<Vec<(&Card<Jokes>, usize)>>();
        vals.sort_by_key(|(card, count)| (*count, *card));
        vals.reverse();
        if let Some((_, count)) = vals.first_mut() {
            if let Some(joker_count) = jokers {
                *count += joker_count
            }
        } else {
            vals.push((&Card::Joker, 5));
        }

        match vals[..] {
            [(a, 5)] => HandType::FiveOfAKind(cards),
            [(a, 4), (b, 1)] => HandType::FourOfAKind(cards),
            [(a, 3), (b, 2)] => HandType::FullHouse(cards),
            [(a, 3), (b, 1), (c, 1)] => HandType::ThreeOfAKind(cards),
            [(a, 2), (b, 2), (c, 1)] => HandType::TwoPair(cards),
            [(a, 2), (b, 1), (c, 1), (d, 1)] => HandType::Pair(cards),
            [(a, 1), (b, 1), (c, 1), (d, 1), (e, 1)] => HandType::HighCard(cards),
            _ => {
                println!("{:?}", vals);
                unreachable!()
            }
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Hand<T> {
    cards: HandType<T>,
    bid: u32,
}

impl FromStr for Hand<NoJoke> {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(" ").context("couldn't get cards and bid")?;
        let mut cards  = cards.chars().map(Card::try_from);

        let cards: [Card<NoJoke>; 5] = [
            cards.next().context("couldn't get card")??,
            cards.next().context("couldn't get card")??,
            cards.next().context("couldn't get card")??,
            cards.next().context("couldn't get card")??,
            cards.next().context("couldn't get card")??,
        ];

        Ok(Self {
            cards: cards.into(),
            bid: bid.parse()?,
        })
    }
}

impl FromStr for Hand<Jokes> {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(" ").context("couldn't get cards and bid")?;
        let mut cards  = cards.chars().map(Card::try_from);

        let cards: [Card<Jokes>; 5] = [
            cards.next().context("couldn't get card")??,
            cards.next().context("couldn't get card")??,
            cards.next().context("couldn't get card")??,
            cards.next().context("couldn't get card")??,
            cards.next().context("couldn't get card")??,
        ];

        Ok(Self {
            cards: cards.into(),
            bid: bid.parse()?,
        })
    }
}

fn part1(input: &str) -> Result<u32> {
    let mut hands = input
        .lines()
        .map(|l| l.parse::<Hand<NoJoke>>())
        .collect::<Result<Vec<_>>>()?;
    hands.sort();

    Ok(hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1) as u32)
        .sum::<u32>())
}

fn part2(input: &str) -> Result<u32> {
    let mut hands = input
        .lines()
        .map(|l| l.parse::<Hand<Jokes>>())
        .collect::<Result<Vec<_>>>()?;
    hands.sort();

    Ok(hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1) as u32)
        .sum::<u32>())
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");

    println!("part1: {}", part1(input)?);
    println!("part2: {}", part2(input)?);

    Ok(())
}

#[test]
fn part1_works() {
    let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    assert_eq!(part1(input).unwrap(), 6440);
}
