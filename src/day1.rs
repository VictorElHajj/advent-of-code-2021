#[aoc_generator(day1)]
pub fn generator_part1(input: &str) -> Vec<usize> {
  let parsed = input.as_bytes().split(|b| *b == b'\n').map(|l|
    l.iter().fold(0, |tot, c| tot * 10 + (*c as usize - '0' as usize))
  ).collect();
  parsed
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<usize>) -> Result<usize, &str> {
    let mut increases = 0;
    for window in input.windows(2) {
     if window[1] > window[0] {
       increases += 1;
     } 
    }
    Ok(increases)
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<usize>) -> Result<usize, &str> {
    let mut increases = 0;
    for window in input.windows(4) {
      let a: usize = window[0..=2].iter().sum();
      let b: usize = window[1..=3].iter().sum();
      if b > a {
        increases += 1;
      }
    }
    Ok(increases)
}
