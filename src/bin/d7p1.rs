use itertools::Itertools;

#[derive(Clone, Copy)]
enum Cell {
    Splitter(bool),
    Empty,
    Beam,
    Source,
}

struct Grid(Vec<Vec<Cell>>);

impl Grid {
    fn new(input: String) -> Self {
        let cells = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '^' => Cell::Splitter(false),
                        '.' => Cell::Empty,
                        'S' => Cell::Source,
                        _ => panic!(),
                    })
                    .collect_vec()
            })
            .collect_vec();
        Self(cells)
    }

    fn source(&self) -> Option<(usize, usize)> {
        self.0.iter().enumerate().find_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, cell)| {
                if matches!(cell, Cell::Source) {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
    }

    fn run(&mut self) {
        let (mut x, _) = self.source().unwrap();
        x += 1;
        while x < self.0.len() {
            for i in 0..self.0[0].len() {
                if matches!(self.0[x][i], Cell::Empty)
                    && matches!(self.0[x - 1][i], Cell::Beam | Cell::Source)
                {
                    self.0[x][i] = Cell::Beam;
                } else if matches!(self.0[x][i], Cell::Splitter(_))
                    && matches!(self.0[x - 1][i], Cell::Beam | Cell::Source)
                {
                    let tmp = self.0[x].get_mut(i - 1).unwrap();
                    if matches!(tmp, Cell::Beam | Cell::Empty) {
                        *tmp = Cell::Beam;
                    }
                    let tmp = self.0[x].get_mut(i + 1).unwrap();
                    if matches!(tmp, Cell::Beam | Cell::Empty) {
                        *tmp = Cell::Beam;
                    }
                    self.0[x][i] = Cell::Splitter(true);
                }
            }
            x += 1;
        }
    }

    fn count_active_splitters(&self) -> usize {
        self.0
            .iter()
            .flatten()
            .copied()
            .filter(|&cell| matches!(cell, Cell::Splitter(true)))
            .count()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut grid = Grid::new(input);
    grid.run();
    println!("{}", grid.count_active_splitters());
    Ok(())
}
