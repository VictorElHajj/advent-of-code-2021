#[aoc_generator(day2)]
pub fn generator_part2(input: &str) -> Vec<Command> {
  let commands = input.as_bytes().split(| b| *b == '\n' as u8).map(|bytes| {
    let mut halves = bytes.split(|b| *b == ' ' as u8);
    let name = halves.next().expect("Failed to parse line.");
    let dist = halves.next().unwrap().iter().fold(0, |tot, c| tot * 10 + (*c as usize - '0' as usize));
    match name[0] as char {
        'f' => Command::Forward(dist),
        'u' => Command::Up(dist),
        'd' => Command::Down(dist),
        _ => panic!("Unexpected command name {:?}", name),
    }
  }).collect();
  commands
}

pub enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

pub struct Sub {
    depth: usize,
    horizontal_position: usize,
    aim: usize
}

impl Sub {
    fn new() -> Self {
        Sub {
            depth: 0,
            horizontal_position: 0,
            aim: 0,
        }
    }
}

#[aoc(day2, part1)]
pub fn part1(commands: &Vec<Command>) -> usize {
    let mut sub = Sub::new();
    for c in commands {
        match c {
            Command::Forward(n) => sub.horizontal_position += n,
            Command::Up(n) => sub.depth -= n,
            Command::Down(n) => sub.depth += n,
        }
    } 
    sub.horizontal_position * sub.depth
}

#[aoc(day2, part2)]
pub fn part2(commands: &Vec<Command>) -> usize {
    let mut sub = Sub::new();
    for c in commands {
        match c {
            Command::Forward(n) => {
                sub.horizontal_position += n;
                sub.depth += sub.aim * n;
            },
            Command::Up(n) => sub.aim -= n,
            Command::Down(n) => sub.aim += n,
        }
    } 
    sub.horizontal_position * sub.depth
}