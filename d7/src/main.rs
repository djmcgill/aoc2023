#![feature(slice_group_by)]

use std::time::Instant;

// const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../real.txt");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum P1Card {
    A,
    K,
    Q,
    J,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
}
impl P1Card {
    fn parse(c: char) -> Self {
        match c {
            'A' => P1Card::A,
            'K' => P1Card::K,
            'Q' => P1Card::Q,
            'J' => P1Card::J,
            'T' => P1Card::T,
            '9' => P1Card::N9,
            '8' => P1Card::N8,
            '7' => P1Card::N7,
            '6' => P1Card::N6,
            '5' => P1Card::N5,
            '4' => P1Card::N4,
            '3' => P1Card::N3,
            '2' => P1Card::N2,
            _ => unreachable!("char: '{c}'"),
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum P2Card {
    A,
    K,
    Q,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
    J,
}
impl P2Card {
    fn parse(c: char) -> Self {
        match c {
            'A' => P2Card::A,
            'K' => P2Card::K,
            'Q' => P2Card::Q,
            'J' => P2Card::J,
            'T' => P2Card::T,
            '9' => P2Card::N9,
            '8' => P2Card::N8,
            '7' => P2Card::N7,
            '6' => P2Card::N6,
            '5' => P2Card::N5,
            '4' => P2Card::N4,
            '3' => P2Card::N3,
            '2' => P2Card::N2,
            _ => unreachable!("char: '{c}'"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}
fn identify_pre<C: Ord + Eq>(mut s: Vec<C>) -> Vec<usize> {
    s.sort();
    let mut group_len = s[..]
        .group_by(|a, b| a == b)
        .map(|g| g.len())
        .collect::<Vec<_>>();
    group_len.sort();
    group_len
}
impl HandType {
    fn identify_p1(s: Vec<P1Card>) -> HandType {
        let group_len = identify_pre(s);
        match group_len[..] {
            [5] => HandType::FiveKind,
            [1, 4] => HandType::FourKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!("hand type: {:?}", group_len),
        }
    }
    fn identify_p2(mut s: Vec<P2Card>) -> HandType {
        s.retain(|x| *x != P2Card::J);
        let group_len = identify_pre(s);
        match group_len[..] {
            [5] | [4] | [3] | [2] | [1] | [] => HandType::FiveKind,
            [1, 4] | [1, 3] | [1, 2] | [1, 1] => HandType::FourKind,
            [2, 3] | [2, 2] => HandType::FullHouse,
            [1, 1, 3] | [1, 1, 2] | [1, 1, 1] => HandType::ThreeKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] | [1, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!("hand type: {:?}", group_len),
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand<C>(HandType, Vec<C>, usize);
impl<C: Clone> Hand<C> {
    fn parse(
        line: &str,
        parse_card: &impl Fn(char) -> C,
        identify: &impl Fn(Vec<C>) -> HandType,
    ) -> Self {
        // TODO: parse fixed length u8 windows
        let mut sections = line.split_ascii_whitespace();
        let hand = sections
            .next()
            .unwrap()
            .chars()
            .map(|c| parse_card(c))
            .collect::<Vec<_>>();

        let rank = identify(hand.clone());
        let bid = usize::from_str_radix(sections.next().unwrap(), 10).unwrap();
        Hand(rank, hand, bid)
    }
}

fn main() {
    let start = Instant::now();
    let p1 = solve::<P1Card>(INPUT, P1Card::parse, HandType::identify_p1);
    let p1_end = Instant::now();
    let p2 = solve::<P2Card>(INPUT, P2Card::parse, HandType::identify_p2);
    let p2_end = Instant::now();
    println!("p1: {p1}");
    println!("p2: {p2}");
    println!("p1: {:?}", p1_end - start); // 459.6µs
    println!("p2: {:?}", p2_end - p1_end); // 449.3µs
}

fn solve<C: Clone + Ord>(
    s: &str,
    parse: impl Fn(char) -> C,
    identify: impl Fn(Vec<C>) -> HandType,
) -> usize {
    let mut ret = s
        .lines()
        .map(move |line| Hand::parse(line, &parse, &identify))
        .collect::<Vec<_>>();
    // TODO: there's a lot of back and forth between vec and iter here
    ret.sort_by(|x, y| x.cmp(&y).reverse());
    ret.into_iter()
        .enumerate()
        .map(|(n, hand)| (n + 1) * hand.2)
        .sum()
}
