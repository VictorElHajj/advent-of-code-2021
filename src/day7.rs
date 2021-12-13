#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<Vec<isize>> {
    input.lines().map(|row| {
        row.split(',').map(|number| {
          number.parse::<isize>().unwrap()  
        }).collect()
    }).collect()
}

fn mean(list: &Vec<isize>) -> isize {
    let mut list = list.clone();
    list.sort();
    if list.len() % 2 == 0 {
        (list[list.len()/2] + list[list.len()/2])/2
    }
    else {
        list[list.len()/2]
    }
}

#[aoc(day7, part1)]
fn part1(crab_subs: &Vec<Vec<isize>>) -> Result<isize, &str> {
    let avg_position = mean(&crab_subs.iter().map(|r| -> isize {
        mean(r) 
    }).collect::<Vec<isize>>());
    let mut dist = 0;
    for row in crab_subs {
        for crab in row {
            dist += (crab - avg_position).abs()
        }
    }
    Ok(dist)
}