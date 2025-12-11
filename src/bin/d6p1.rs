use itertools::Itertools;

fn transpose(x: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut result = vec![vec![0; x.len()]; x[0].len()];
    for i in 0..x.len() {
        for j in 0..x[0].len() {
            result[j][i] = x[i][j];
        }
    }
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let nums = input
        .lines()
        .rev()
        .skip(1)
        .map(|x| {
            x.split_whitespace()
                .map(str::parse::<usize>)
                .map(Result::unwrap)
                .collect_vec()
        })
        .collect_vec();

    let mut nums = transpose(nums).into_iter();
    let mut result = 0;

    input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .for_each(|s| {
            if s == "*" {
                result += nums.next().unwrap().into_iter().product::<usize>();
            } else if s == "+" {
                result += nums.next().unwrap().into_iter().sum::<usize>();
            } else {
                panic!();
            }
        });

    println!("{result:?}");

    Ok(())
}
