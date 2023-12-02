use std::{cmp::max, time::Instant};

// const INPUT: &[u8] = include_bytes!("../test.txt");
const INPUT: &[u8] = include_bytes!("../real.txt");

#[derive(Debug)]
struct Game {
    game_id: usize,
    max_red: u32,
    max_blue: u32,
    max_green: u32,
}
impl Game {
    fn parse(mut input: &[u8], game_id: usize) -> (&[u8], Game) {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        // drop 'Game '
        // drop game number
        // drop ': '
        if game_id < 10 {
            input = &input[5 + 1 + 2..];
        } else if game_id < 100 {
            input = &input[5 + 2 + 2..];
        } else {
            input = &input[5 + 3 + 2..];
        }

        loop {
            let (input_, round) = Round::parse(input);
            input = input_;

            max_red = max(max_red, round.red);
            max_blue = max(max_blue, round.blue);
            max_green = max(max_green, round.green);

            if input.is_empty() || input[0] == b'\n' {
                break;
            }
            input = &input[2..]; // drop '; '
        }
        let game = Game {
            game_id,
            max_red,
            max_blue,
            max_green,
        };
        (input, game)
    }
}
#[derive(Debug)]

struct Round {
    blue: u32,
    red: u32,
    green: u32,
}
impl Round {
    fn parse(mut input: &[u8]) -> (&[u8], Round) {
        let mut output = Round {
            blue: 0,
            red: 0,
            green: 0,
        };

        loop {
            let (input_, int) = parse_int(input);
            match input_[1] {
                b'b' => {
                    output.blue += int;
                    input = &input_[5..];
                }
                b'r' => {
                    output.red += int;
                    input = &input_[4..];
                }
                b'g' => {
                    output.green += int;
                    input = &input_[6..];
                }
                _ => unreachable!(),
            }

            if input.is_empty() || input[0] != b',' {
                break (input, output);
            } else {
                input = &input[2..]; // drop ', '
            }
        }
    }
}
fn parse_int(input: &[u8]) -> (&[u8], u32) {
    let mut ans = 0u32;
    let mut len = 0;

    while (b'0'..=b'9').contains(&input[len]) {
        ans *= 10;
        ans += (input[len] - b'0') as u32;
        len += 1;
    }

    (&input[len..], ans)
}

fn main() {
    let p1_start = Instant::now();

    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    let mut games = vec![];
    let mut input = INPUT;
    let mut game_id = 1;
    while !input.is_empty() {
        let (input_, game) = Game::parse(input, game_id);
        games.push(game);
        game_id += 1;
        input = &input_[1..]; // drop '\n'
    }

    let parse_end = Instant::now();

    let p1: usize = games
        .iter()
        .filter_map(|game| {
            let pred = game.max_red <= red_limit
                && game.max_blue <= blue_limit
                && game.max_green <= green_limit;
            if pred {
                Some(game.game_id)
            } else {
                None
            }
        })
        .sum();

    let p1_end = Instant::now();

    println!("p1: {p1}\n");

    let p2_start = Instant::now();

    let p2: u32 = games
        .iter()
        .map(|game| game.max_red * game.max_blue * game.max_green)
        .sum();

    let p2_end = Instant::now();
    println!("bespoke p2: {p2}");
    println!("parse: {:?}", parse_end - p1_start); // 16.2µs
    println!("p1 no parse: {:?}", p1_end - parse_end); // 600ns
    println!("p2 no parse: {:?}", p2_end - p2_start); // 100ns
}
