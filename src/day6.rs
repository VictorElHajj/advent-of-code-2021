#[aoc_generator(day6)]
pub fn generator(input: &[u8]) -> [u64; 9] {
    let mut fishes = [0; 9];
    for bs in input.split(|b| b == &b',') {
        fishes[(bs.first().unwrap() - b'0') as usize] += 1;
    }
    fishes
}

#[aoc(day6, part1)]
pub fn part1(fishes: &[u64; 9]) -> Result<u64, &str> {
    let mut fishes = fishes.clone();
    let days = 80;
    for _ in 0..days {
        let birthing = fishes[0];
        fishes[0] = fishes[1];
        fishes[1] = fishes[2];
        fishes[2] = fishes[3];
        fishes[3] = fishes[4];
        fishes[4] = fishes[5];
        fishes[5] = fishes[6];
        fishes[6] = fishes[7] + birthing;
        fishes[7] = fishes[8];
        fishes[8] = birthing;
    }
    Ok(fishes.iter().sum::<u64>())
}

#[aoc(day6, part2)]
pub fn part2(fishes: &[u64; 9]) -> Result<u64, &str> {
    let days = 256;
    for _ in 0..days {
        let birthing = fishes[0];
        fishes[0] = fishes[1];
        fishes[1] = fishes[2];
        fishes[2] = fishes[3];
        fishes[3] = fishes[4];
        fishes[4] = fishes[5];
        fishes[5] = fishes[6];
        fishes[6] = fishes[7] + birthing;
        fishes[7] = fishes[8];
        fishes[8] = birthing;
    }
    Ok(fishes.iter().sum::<u64>())
}
