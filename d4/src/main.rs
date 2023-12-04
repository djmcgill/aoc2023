use std::{collections::HashSet, time::Instant};

// const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../real.txt");

fn main() {
    // can't really be bothered to write bespoke parsers today
    let start = Instant::now();
    let p1: u32 = INPUT
        .lines()
        .map(|line| {
            let mut hash_sets = line
                .split(':')
                .skip(1)
                .next()
                .unwrap()
                .split('|')
                .map(collect_numbers);
            let winning_numbers = hash_sets.next().unwrap();
            let my_numbers = hash_sets.next().unwrap();
            let my_wins = winning_numbers.intersection(&my_numbers);
            2_u32.pow(my_wins.count() as u32) / 2
        })
        .sum();
    let p1_end = Instant::now();

    // (original score, number of copies)
    let mut p2_answers = INPUT
        .lines()
        .map(|line| {
            let mut x = line
                .split(':')
                .skip(1)
                .next()
                .unwrap()
                .split('|')
                .map(collect_numbers);
            let w_set = x.next().unwrap();
            let m_set = x.next().unwrap();
            (m_set.intersection(&w_set).count(), 1)
        })
        .collect::<Vec<_>>();

    // todo: 2 passes is unnecessary we could just do this in advance in a single pass
    let mut p2 = 0;
    for i in 0..p2_answers.len() {
        let (wins, copies) = p2_answers[i];
        for j in 1..=wins {
            p2_answers[i + j].1 += copies;
        }
        p2 += copies;
    }
    let p2_end = Instant::now();

    println!("p1: {p1}");
    println!("p1: {:?}", p1_end - start); // 509µs
    println!("p2: {p2}");
    println!("p2: {:?}", p2_end - p1_end); // 450µs
}

fn collect_numbers(ns: &str) -> HashSet<u32> {
    ns.split(' ')
        .filter_map(|x| {
            if x.trim().is_empty() {
                None
            } else {
                Some(u32::from_str_radix(x, 10).unwrap())
            }
        })
        .collect::<HashSet<_>>()
}
