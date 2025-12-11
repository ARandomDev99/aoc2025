use itertools::Itertools;

fn transpose<T: Clone>(x: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut result = vec![Vec::with_capacity(x.len()); x.iter().map(|i| i.len()).max().unwrap()];
    for i in 0..x.len() {
        for j in 0..x[i].len() {
            result[j].push(x[i][j].clone());
        }
    }
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let nums = transpose(
        input
            .lines()
            .rev()
            .skip(1)
            .collect_vec()
            .into_iter()
            .rev()
            .map(|x| x.chars().collect_vec())
            .collect_vec(),
    )
    .into_iter()
    .rev()
    .map(|x| x.into_iter().join("").trim().to_owned())
    .collect_vec()
    .split(String::is_empty)
    .map(|x| {
        x.iter()
            .map(|x| x.parse::<usize>())
            .map(Result::unwrap)
            .collect_vec()
    })
    .collect_vec();

    let result = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .rev()
        .zip(nums)
        .map(|(s, nums)| {
            if s == "*" {
                nums.into_iter().product::<usize>()
            } else if s == "+" {
                nums.into_iter().sum::<usize>()
            } else {
                panic!();
            }
        })
        .sum::<usize>();

    println!("{result:?}");

    Ok(())
}
