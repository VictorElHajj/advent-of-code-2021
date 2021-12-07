use std::collections::HashMap;

pub struct Line {
    start: Point,
    end: Point,
}
#[derive(Debug)]
pub struct Point {
    x: isize,
    y: isize,
}

#[aoc_generator(day5)]
pub fn generator_part1(input: &str) -> Vec<Line> {
    let parsed = input.lines().map(|line| {
        let (start, end) = line.split_once(" -> ").expect("Could not split start and end");
        let (start_x, start_y) = start.split_once(',').expect("Could nto split start line");
        let (end_x, end_y) = end.split_once(',').expect("Could not split end line");
        let start: Point = Point { x: start_x.parse().unwrap(), y: start_y.parse().unwrap() };
        let end: Point = Point { x: end_x.parse().unwrap(), y: end_y.parse().unwrap() };
        Line { start, end }
    }).collect();
    parsed
}

#[aoc(day5, part1)]
pub fn part1(lines: &Vec<Line>) -> Result<isize, &str> {
    let mut hm: HashMap<(isize, isize), isize> = HashMap::new();
    for line in lines {
        // Only consider horizontal and vertical lines
        if line.start.x == line.end.x {
            let x = line.start.x;
            let range = if line.start.y < line.end.y {line.start.y..=line.end.y} else {line.end.y..=line.start.y};
            for y in range {
                let entry: &mut isize = hm.entry((x,y)).or_insert(0);
                *entry += 1;
            }
        } else if line.start.y == line.end.y {
            let y = line.start.y;
            let range = if line.start.x < line.end.x {line.start.x..=line.end.x} else {line.end.x..=line.start.x};
            for x in range {
                let entry: &mut isize = hm.entry((x,y)).or_insert(0);
                *entry += 1;
            }

        }
    }
    Ok(hm.values().fold(0, |acc, v| if v > &1 {acc + 1} else {acc}))
}

#[aoc(day5, part2)]
pub fn part2(lines: &Vec<Line>) -> Result<isize, &str> {
    let mut hm: HashMap<(isize, isize), isize> = HashMap::new();
    for line in lines {
        // Only consider horizontal and vertical lines
        if line.start.x == line.end.x {
            let x = line.start.x;
            let range = if line.start.y < line.end.y {line.start.y..=line.end.y} else {line.end.y..=line.start.y};
            for y in range {
                let entry: &mut isize = hm.entry((x,y)).or_insert(0);
                *entry += 1;
            }
        } else if line.start.y == line.end.y {
            let y = line.start.y;
            let range = if line.start.x < line.end.x {line.start.x..=line.end.x} else {line.end.x..=line.start.x};
            for x in range {
                let entry: &mut isize = hm.entry((x,y)).or_insert(0);
                *entry += 1;
            }

        }
        // Diagonals
        else {
            let dx: isize = if line.start.x < line.end.x {1} else {-1};
            let dy: isize = if line.start.y < line.end.y {1} else {-1};
            let steps = (line.start.y - line.end.y).abs();
            let x = line.start.x;
            let y = line.start.y;
            for i in 0..=steps {
                let entry: &mut isize = hm.entry((x+dx*i,y+dy*i)).or_insert(0);
                *entry += 1;
            }
        }
    }
    Ok(hm.values().fold(0, |acc, v| if v > &1 {acc + 1} else {acc}))
}