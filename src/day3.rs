
#[aoc_generator(day3)]
pub fn generator_part1(input: &[u8]) -> Vec<Vec<u8>> {
    let parsed = input.split(|b| *b == b'\n').map(|b| b.to_owned()).collect();
    parsed
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<Vec<u8>>) -> Result<u32, &str> {
    let mut count = [[0u16; 2]; 12];
    let mut gamma = 0;
    for chars in input {
        for (i, c) in chars.iter().enumerate() {
            match c {
                b'0' => count[i][0] += 1,
                b'1' => count[i][1] += 1,
                _ => panic!("Invalid character {:?}", c)
            }
        }
    }
    for [zeroes, ones] in count.iter() {
        gamma *= 2;
        if ones > zeroes {
            gamma += 1;
        }
    }
    let epsilon = 2u32.pow(12)-1-gamma;
    Ok(gamma*epsilon)
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<Vec<u8>>) -> Result<u32, &str> {
    // This is a mess but was in a hurry
    // Find Oxygen rating
    let mut i1 = input.clone();
    for i in 0..12 {
        let mut count = [0u16; 2];
        for chars in i1.iter() {
                match chars[i] {
                    b'0' => count[0] += 1,
                    b'1' => count[1] += 1,
                    _ => panic!("Invalid character {:?}", chars[i])
                }
        }
        let filter = if count[1] >= count[0] { b'1' } else { b'0' };
        i1 = i1.iter().filter(|bytes| bytes[i] == filter).cloned().collect();
        if i1.len() == 1 {
            break;
        }
    }
    // Find CO2 Scrubber
    let mut i2 = input.clone();
    for i in 0..12 {
        let mut count = [0u16; 2];
        for chars in i2.iter() {
                match chars[i] {
                    b'0' => count[0] += 1,
                    b'1' => count[1] += 1,
                    _ => panic!("Invalid character {:?}", chars[i])
                }
        }
        let filter = if count[1] < count[0] { b'1' } else { b'0' };
        i2 = i2.iter().filter(|bytes| bytes[i] == filter).cloned().collect();
        if i2.len() == 1 {
            break;
        }
    }
    let oxygen: u32 = i1[0].iter().fold(0, |tot, c| tot * 2 + (*c - b'0') as u32);
    let co2: u32 = i2[0].iter().fold(0, |tot, c| tot * 2 + (*c - b'0') as u32);
    Ok(oxygen * co2)
}