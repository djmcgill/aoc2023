use std::time::Instant;

// const INPUT: &str = include_str!("../test1.txt");
// const INPUT: &str = include_str!("../test2.txt");
const INPUT: &[u8] = include_bytes!("../real.txt");
// const INPUT: &str = "two1nine\neightwothree\n";

fn main() {
    // let p1: u32 = INPUT
    //     .lines()
    //     .map(|line| {
    //         let mut numbers = line.chars().filter(|c| c.is_numeric());
    //         let first = numbers.next().unwrap();
    //         let last = numbers.last().unwrap_or(first);
    //         first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
    //     })
    //     .sum();
    // println!("p1: {p1}");

    let before_p2 = Instant::now();

    let mut p2 = 0;
    let mut state_machine = TextParsingStateMachine::new();
    let mut input = INPUT.into_iter();

    while let Some(c) = input.next() {
        loop {
            let reset = state_machine.advance(*c);
            match reset {
                // we matched, but might still need the last digit
                StateMachineResult::Match => {
                    state_machine.buffer.clear();
                    state_machine.advance(*c);
                    break;
                }
                // we found something invalid, clear the buffer 1-by-1 until we match or exhaust the buffer
                StateMachineResult::Fail => {
                    if state_machine.buffer.len() > 0 {
                        state_machine.buffer.remove(0); // todo: remove(0) on vec is bad
                                                        // continue;
                    } else {
                        break;
                    }
                }
                // unclear, next digit
                StateMachineResult::Inconclusive => {
                    break;
                }
                StateMachineResult::Newline => {
                    let first = state_machine.digits[0];
                    let last = *state_machine.digits.last().unwrap_or(&first);
                    // dbg!((first, last));
                    p2 += (first * 10 + last) as u32;
                    state_machine.buffer.clear();
                    state_machine.digits.clear();
                    break;
                }
            }
        }
    }
    let after_p2 = Instant::now();
    let time = after_p2 - before_p2;

    // 255.7µs
    println!("p2: {p2} ({:#?})", time);
}

enum StateMachineResult {
    Inconclusive,
    Match,
    Fail,
    Newline,
}

#[derive(Debug)]
struct TextParsingStateMachine {
    buffer: Vec<u8>,
    digits: Vec<u8>,
}
impl TextParsingStateMachine {
    fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(4),
            digits: Vec::with_capacity(2),
        }
    }
    fn advance(&mut self, c: u8) -> StateMachineResult {
        match self.buffer[..] {
            _ if c == b'\n' => {
                return StateMachineResult::Newline;
            }
            _ if (b'1'..=b'9').contains(&c) => {
                self.digits.push(c - b'0');
                return StateMachineResult::Match;
            }
            [b'o', b'n'] if c == b'e' => {
                self.digits.push(1);
                return StateMachineResult::Match;
            }
            [b't', b'w'] if c == b'o' => {
                self.digits.push(2);
                return StateMachineResult::Match;
            }
            [b't', b'h', b'r', b'e'] if c == b'e' => {
                self.digits.push(3);
                return StateMachineResult::Match;
            }
            [b'f', b'o', b'u'] if c == b'r' => {
                self.digits.push(4);
                return StateMachineResult::Match;
            }
            [b'f', b'i', b'v'] if c == b'e' => {
                self.digits.push(5);
                return StateMachineResult::Match;
            }
            [b's', b'i'] if c == b'x' => {
                self.digits.push(6);
                return StateMachineResult::Match;
            }
            [b's', b'e', b'v', b'e'] if c == b'n' => {
                self.digits.push(7);
                return StateMachineResult::Match;
            }
            [b'e', b'i', b'g', b'h'] if c == b't' => {
                self.digits.push(8);
                return StateMachineResult::Match;
            }
            [b'n', b'i', b'n'] if c == b'e' => {
                self.digits.push(9);
                return StateMachineResult::Match;
            }

            [] if [b'o', b't', b'f', b's', b'e', b'n'].contains(&c) => {
                self.buffer.push(c);
            }
            [b'o'] if c == b'n' => self.buffer.push(c),
            [b't'] if c == b'w' || c == b'h' => self.buffer.push(c),
            [b't', b'h'] if c == b'r' => self.buffer.push(c),
            [b't', b'h', b'r'] if c == b'e' => self.buffer.push(c),
            [b'f'] if c == b'o' || c == b'i' => self.buffer.push(c),
            [b'f', b'o'] if c == b'u' => self.buffer.push(c),
            [b'f', b'i'] if c == b'v' => self.buffer.push(c),
            [b's'] if c == b'i' || c == b'e' => self.buffer.push(c),
            [b's', b'e'] if c == b'v' => self.buffer.push(c),
            [b's', b'e', b'v'] if c == b'e' => self.buffer.push(c),
            [b'e'] if c == b'i' => self.buffer.push(c),
            [b'e', b'i'] if c == b'g' => self.buffer.push(c),
            [b'e', b'i', b'g'] if c == b'h' => self.buffer.push(c),
            [b'n'] if c == b'i' => self.buffer.push(c),
            [b'n', b'i'] if c == b'n' => self.buffer.push(c),
            _ => return StateMachineResult::Fail,
        }
        StateMachineResult::Inconclusive
    }
}
