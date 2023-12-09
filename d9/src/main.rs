use std::time::Instant;

// const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../real.txt");

fn main() {
    let start = Instant::now();
    let sequences = INPUT
        .lines()
        .map(|line| line.split(' ').map(|n| i32::from_str_radix(n, 10).unwrap()));
    let mut p1 = 0;
    let mut p2 = 0;

    sequences.for_each(|line| {
        let mut n = 0;
        let mut seqs = vec![line.collect::<Vec<_>>()];
        while !&seqs
            .last()
            .unwrap()
            .iter()
            .all(|&x| x == seqs.last().unwrap()[0])
        {
            n += 1;
            let x = differentiate(seqs.last().unwrap());
            seqs.push(x);
        }
        // dbg!(n);
        // dbg!(&seqs);

        let mut firsts = vec![0; n + 1];
        firsts[n] = seqs.last().unwrap()[0];
        for x in (1..=n).rev() {
            let previous_seq_last = *seqs[x - 1].last().unwrap();
            let diff_seq_last = *seqs[x].last().unwrap();
            seqs[x - 1].push(previous_seq_last + diff_seq_last);

            let previous_seq_first = seqs[x - 1][0];
            let diff_first = firsts[x];
            // dbg!(x);
            firsts[x - 1] = previous_seq_first - diff_first;
        }
        // dbg!(&seqs);

        // dbg!(&firsts);

        p1 += *seqs[0].last().unwrap();
        p2 += firsts[0];
    });

    let p1_end = Instant::now();
    println!("p1: {p1}");
    println!("p2: {p2}");
    println!("p1+p2 combined: {:?}", p1_end - start);
}

fn differentiate(v: &Vec<i32>) -> Vec<i32> {
    let mut ret = Vec::with_capacity(v.len() - 1);
    for i in 0..v.len() - 1 {
        ret.push(v[i + 1] - v[i]);
    }
    ret
}
