use itertools::Itertools;

enum Direction {
    Left,
    Right,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let lines = input
        .lines()
        .map(|line| {
            let direction = if &line[..1] == "L" {
                Direction::Left
            } else if &line[..1] == "R" {
                Direction::Right
            } else {
                panic!();
            };
            let distance = line[1..].parse::<i32>().unwrap();
            (direction, distance)
        })
        .collect_vec();
    let mut x = 50;
    let mut counter = 0;
    for (dir, i) in lines {
        counter += i / 100;
        match dir {
            Direction::Left => {
                let tmp = (x - i).rem_euclid(100);
                if tmp > x && x != 0 {
                    counter += 1;
                }
                x = tmp;
                if x == 0 {
                    counter += 1;
                }
            }
            Direction::Right => {
                let tmp = (x + i).rem_euclid(100);
                if tmp < x {
                    counter += 1;
                }
                x = tmp;
            }
        }
    }
    println!("{}", counter);
    Ok(())
}
