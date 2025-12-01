use std::fs::File;
use std::io::{self, BufRead};
enum Rotate {
    Left(u32),
    Right(u32),
}
fn parse() -> Vec<Rotate> {
    let path = "data/day1.txt";
    // let path = "data/test_day1.txt";
    let file = File::open(path).unwrap();

    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    let mut res = vec![];
    for line in &lines {
        let dir = line.chars().next().unwrap();
        let num = line[1..].parse::<u32>().unwrap();
        if dir == 'L' {
            res.push(Rotate::Left(num));
        } else {
            res.push(Rotate::Right(num));
        }
    }
    res
}

fn rotate(rotation: &Rotate, mut dial: u32) -> u32 {
    match rotation {
        Rotate::Left(n) => {
            dial = ((dial + 100) - (*n % 100)) % 100;
        }
        Rotate::Right(n) => {
            dial = (dial + (*n % 100)) % 100;
        }
    }
    dial
}

fn count0(rotation: &Rotate, mut dial: u32) -> u32 {
    let mut count = 0;
    match rotation {
        Rotate::Left(n) => {
            count = *n / 100;
            if dial != 0 && dial < (*n % 100) {
                count += 1;
            }
        }
        Rotate::Right(n) => {
            count = *n / 100;
            if dial + (*n % 100) > 100 {
                count += 1;
            }
        }
    }
    count
}

fn part1() -> u32 {
    let rotations = parse();
    let mut res = 0;
    let mut dial = 50;
    for rotation in rotations {
        dial = rotate(&rotation, dial);
        if dial == 0 {
            res += 1;
        }
    }
    res
}

fn part2() -> u32 {
    let rotations = parse();
    let mut res = 0;
    let mut dial = 50;
    for rotation in rotations {
        res += count0(&rotation, dial);
        // println!("{}", count0(&rotation, dial));
        dial = rotate(&rotation, dial);
        // println!("{}", dial);
        // println!("----");
        if dial == 0 {
            res += 1;
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let mut dial = 50;
        dial = rotate(&Rotate::Left(68), dial);
        assert_eq!(dial, 82);
        dial = rotate(&Rotate::Left(30), dial);
        assert_eq!(dial, 52);
        parse();
    }
    #[test]
    fn run_part1() {
        let res = part1();
        println!("{}", res);
    }
    #[test]

    fn run_part2() {
        let res = part2();
        println!("{}", res);
    }
}
