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

    let mut i = 0;
    while i < INPUT.len() {
        match state_machine.advance(INPUT[i]) {
            // we matched, but might still need the last digit so don't advance
            StateMachineResult::MatchLetter => {
                state_machine.parse_state = WordParseState::Empty;
            }
            // we matched, and don't need the last digit so DO advance
            StateMachineResult::MatchDigit => {
                state_machine.parse_state = WordParseState::Empty;
                i += 1;
            }
            // we found something invalid, clear the buffer 1-by-1 until we match or exhaust the buffer
            StateMachineResult::Fail => {
                if let WordParseState::Empty = state_machine.parse_state {
                    i += 1;
                } else {
                    state_machine.reverse();
                }
            }
            // unclear, next digit
            StateMachineResult::Inconclusive => {
                i += 1;
            }
            StateMachineResult::Newline => {
                let first = state_machine.first.unwrap();
                let last = state_machine.last.unwrap_or(first);
                p2 += (first * 10 + last) as u32;
                state_machine.parse_state = WordParseState::Empty;
                state_machine.first = None;
                state_machine.last = None;
                i += 1;
            }
        }
    }
    let after_p2 = Instant::now();
    let time = after_p2 - before_p2;

    // 165.9µs
    println!("p2: {p2} ({:#?})", time);
}

#[derive(Debug)]
enum StateMachineResult {
    Inconclusive,
    MatchLetter,
    MatchDigit,
    Fail,
    Newline,
}

#[derive(Debug)]
struct TextParsingStateMachine {
    parse_state: WordParseState,
    first: Option<u8>,
    last: Option<u8>,
}
impl TextParsingStateMachine {
    fn new() -> Self {
        Self {
            parse_state: WordParseState::Empty,
            first: None,
            last: None,
        }
    }
    fn record_digit(&mut self, c: u8) {
        if self.first.is_none() {
            self.first = Some(c);
        } else {
            self.last = Some(c);
        }
    }

    fn reverse(&mut self) {
        // okay so in the buffer version, this would drop the head and turn "nin" into "in".
        // But in this version, we're limited in how we _can_ drop the head. So only do it if
        // it makes sense
        self.parse_state = match self.parse_state {
            WordParseState::On => WordParseState::N,
            WordParseState::Thre => WordParseState::E,
            WordParseState::Fo => WordParseState::O,
            WordParseState::Se => WordParseState::E,
            WordParseState::Seve => WordParseState::E,
            WordParseState::Nin => WordParseState::N,
            _ => WordParseState::Empty,
        };
    }

    fn advance(&mut self, c: u8) -> StateMachineResult {
        match self.parse_state {
            _ if (b'1'..=b'9').contains(&c) => {
                self.record_digit(c - b'0');
                return StateMachineResult::MatchDigit;
            }
            _ if c == b'\n' => {
                return StateMachineResult::Newline;
            }

            WordParseState::Empty if c == b'o' => self.parse_state = WordParseState::O,
            WordParseState::Empty if c == b't' => self.parse_state = WordParseState::T,
            WordParseState::Empty if c == b'f' => self.parse_state = WordParseState::F,
            WordParseState::Empty if c == b's' => self.parse_state = WordParseState::S,
            WordParseState::Empty if c == b'e' => self.parse_state = WordParseState::E,
            WordParseState::Empty if c == b'n' => self.parse_state = WordParseState::N,

            WordParseState::O if c == b'n' => self.parse_state = WordParseState::On,
            WordParseState::T if c == b'w' => self.parse_state = WordParseState::Tw,
            WordParseState::T if c == b'h' => self.parse_state = WordParseState::Th,
            WordParseState::F if c == b'o' => self.parse_state = WordParseState::Fo,
            WordParseState::F if c == b'i' => self.parse_state = WordParseState::Fi,
            WordParseState::S if c == b'i' => self.parse_state = WordParseState::Si,
            WordParseState::S if c == b'e' => self.parse_state = WordParseState::Se,
            WordParseState::E if c == b'i' => self.parse_state = WordParseState::Ei,
            WordParseState::N if c == b'i' => self.parse_state = WordParseState::Ni,

            WordParseState::Th if c == b'r' => self.parse_state = WordParseState::Thr,
            WordParseState::Thr if c == b'e' => self.parse_state = WordParseState::Thre,
            WordParseState::Fo if c == b'u' => self.parse_state = WordParseState::Fou,
            WordParseState::Fi if c == b'v' => self.parse_state = WordParseState::Fiv,
            WordParseState::Se if c == b'v' => self.parse_state = WordParseState::Sev,
            WordParseState::Sev if c == b'e' => self.parse_state = WordParseState::Seve,
            WordParseState::Ei if c == b'g' => self.parse_state = WordParseState::Eig,
            WordParseState::Eig if c == b'h' => self.parse_state = WordParseState::Eigh,
            WordParseState::Ni if c == b'n' => self.parse_state = WordParseState::Nin,

            WordParseState::On if c == b'e' => {
                self.record_digit(1);
                return StateMachineResult::MatchLetter;
            }
            WordParseState::Tw if c == b'o' => {
                self.record_digit(2);
                return StateMachineResult::MatchLetter;
            }
            WordParseState::Thre if c == b'e' => {
                self.record_digit(3);
                return StateMachineResult::MatchLetter;
            }
            WordParseState::Fou if c == b'r' => {
                self.record_digit(4);
                return StateMachineResult::MatchLetter;
            }
            WordParseState::Fiv if c == b'e' => {
                self.record_digit(5);
                return StateMachineResult::MatchLetter;
            }
            WordParseState::Si if c == b'x' => {
                self.record_digit(6);
                return StateMachineResult::MatchLetter;
            }
            WordParseState::Seve if c == b'n' => {
                self.record_digit(7);
                return StateMachineResult::MatchLetter;
            }
            WordParseState::Eigh if c == b't' => {
                self.record_digit(8);
                return StateMachineResult::MatchLetter;
            }
            WordParseState::Nin if c == b'e' => {
                self.record_digit(9);
                return StateMachineResult::MatchLetter;
            }

            _ => return StateMachineResult::Fail,
        }
        StateMachineResult::Inconclusive
    }
}

#[derive(Debug)]
enum WordParseState {
    Empty,
    O,
    On,
    T,
    Tw,
    Th,
    Thr,
    Thre,
    F,
    Fo,
    Fou,
    Fi,
    Fiv,
    S,
    Si,
    Se,
    Sev,
    Seve,
    E,
    Ei,
    Eig,
    Eigh,
    N,
    Ni,
    Nin,
}
