use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let lines = input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|i| i - b'0')
                .map(usize::from)
                .rev()
                .collect_vec()
        })
        .collect_vec();
    let result = lines
        .into_iter()
        .map(|line| {
            let max_idx = line.iter().position_max().unwrap();
            if max_idx == 0 {
                line.iter().skip(1).max().unwrap() * 10 + line[0]
            } else {
                line.iter().take(max_idx).max().unwrap() + line[max_idx] * 10
            }
        })
        .sum::<usize>();
    println!("{result:?}");
    Ok(())
}
