use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let input = input.trim();
    let ranges = input
        .split(",")
        .map(|range| {
            range
                .split('-')
                .map(|s| s.parse::<u128>().unwrap())
                .collect_tuple::<(u128, u128)>()
                .unwrap()
        })
        .collect_vec();
    let result = ranges
        .into_iter()
        .map(|(lo, hi)| {
            (lo..=hi)
                .filter(|x| {
                    let s = x.to_string();
                    s.len() % 2 == 0 && s[..s.len() / 2] == s[s.len() / 2..]
                })
                .sum::<u128>()
        })
        .sum::<u128>();
    println!("{}", result);
    Ok(())
}
