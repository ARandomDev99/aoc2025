use itertools::Itertools;

const PERMUTATION_LENGTH: usize = 12;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let lines = input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|i| i - b'0')
                .map(usize::from)
                .collect_vec()
        })
        .collect_vec();
    let line_len = lines[0].len();
    let result = lines
        .into_iter()
        .map(|line| {
            let mut max = vec![];
            let mut prev = 0;
            for perm_idx in 1..=PERMUTATION_LENGTH {
                let (max_idx, max_val) = line
                    .iter()
                    .copied()
                    .enumerate()
                    .skip(prev)
                    .take(line_len - PERMUTATION_LENGTH - prev + perm_idx)
                    .reduce(|acc, val| if val.1 > acc.1 { val } else { acc })
                    .unwrap();
                max.push(max_val);
                prev = max_idx + 1;
            }
            max.into_iter()
                .enumerate()
                .map(|(i, v)| v * 10usize.pow((PERMUTATION_LENGTH - 1 - i) as u32))
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{result:?}");
    Ok(())
}
