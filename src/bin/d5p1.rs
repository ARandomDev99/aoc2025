use itertools::Itertools;

#[derive(Debug)]
struct Range(usize, usize);

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Range(start, end)
    }
    fn contains(&self, x: usize) -> bool {
        self.0 <= x && self.1 >= x
    }
    fn overlaps(&self, other: &Range) -> bool {
        (self.0 <= other.1 && other.1 <= self.1) || (other.0 <= self.1 && self.1 <= other.1)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut ranges = input
        .lines()
        .take_while(|x| !x.is_empty())
        .map(|x| {
            let (start, end) = x
                .split("-")
                .map(str::parse)
                .map(Result::unwrap)
                .collect_tuple()
                .unwrap();
            Range::new(start, end)
        })
        .collect_vec();

    let mut overlap = true;

    while overlap {
        overlap = false;
        for (x, y) in (0..ranges.len()).cartesian_product(0..ranges.len()) {
            if x != y && ranges[x].overlaps(&ranges[y]) {
                ranges[x] = Range::new(ranges[x].0.min(ranges[y].0), ranges[x].1.max(ranges[y].1));
                ranges.remove(y);
                overlap = true;
                break;
            }
        }
    }

    let result = input
        .lines()
        .skip_while(|s| !s.is_empty())
        .skip(1)
        .map(str::parse)
        .map(Result::unwrap)
        .filter(|id| ranges.iter().any(|range| range.contains(*id)))
        .count();
    println!("{}", result);

    Ok(())
}
