use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

type Card = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard(Card, Card, Card, Card, Card),
    OnePair(Card, Card, Card, Card, Card),
    TwoPair(Card, Card, Card, Card, Card),
    ThreeOfAKind(Card, Card, Card, Card, Card),
    FullHouse(Card, Card, Card, Card, Card),
    FourOfAKind(Card, Card, Card, Card, Card),
    FiveOfAKind(Card, Card, Card, Card, Card),
}

impl HandType {
    fn new(input: Vec<u8>) -> Self {
        let cards: Vec<Card> = input
            .iter()
            .map(|c| match c {
                b'A' => 14,
                b'K' => 13,
                b'Q' => 12,
                b'J' => 11,
                b'T' => 10,
                b'0'..=b'9' => c - b'0',
                _ => 0,
            })
            .collect();
        let grouped_cards = cards
            .iter()
            .sorted()
            .group_by(|c| *c)
            .into_iter()
            .map(|(_, g)| g.collect_vec())
            .sorted_by_key(|v: &Vec<&u8>| v.len())
            .collect::<Vec<Vec<_>>>();
        let clasi = &grouped_cards
            .iter()
            .map(|v| v.len())
            .collect::<Vec<usize>>();
        match &clasi[..] {
            [5] => HandType::FiveOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4]),
            [_, 4] => HandType::FourOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4]),
            [2, 3] => HandType::FullHouse(cards[0], cards[1], cards[2], cards[3], cards[4]),
            [_, _, 3] => HandType::ThreeOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4]),
            [_, 2, 2] => HandType::TwoPair(cards[0], cards[1], cards[2], cards[3], cards[4]),
            [_, _, _, 2] => HandType::OnePair(cards[0], cards[1], cards[2], cards[3], cards[4]),
            _ => HandType::HighCard(cards[0], cards[1], cards[2], cards[3], cards[4]),
        }
    }
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    bet: usize,
}

impl Hand {
    fn new(line: &str) -> Self {
        let mut parts = line.split(' ');
        let hand_type = HandType::new(parts.next().unwrap().bytes().collect::<Vec<u8>>());
        let bet = parts.next().unwrap().parse::<usize>().unwrap();
        Hand { hand_type, bet }
    }
}

fn part1(input: &str) -> usize {
    let mut hands: Vec<Hand> = input.lines().map(Hand::new).collect();
    hands.sort_by_key(|h| h.hand_type);
    println!("{:#?}", hands);
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bet)
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, 6440);
    }
}
