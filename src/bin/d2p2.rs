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
                    'a: for i in 1..=s.len() / 2 {
                        if s.len() % i != 0 {
                            continue;
                        }
                        for j in 0..(s.len() / i) - 1 {
                            if s[j * i..(j + 1) * i] != s[(j + 1) * i..(j + 2) * i] {
                                continue 'a;
                            }
                        }
                        return true;
                    }
                    false
                })
                .sum::<u128>()
        })
        .sum::<u128>();
    println!("{}", result);
    Ok(())
}
