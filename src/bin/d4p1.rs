use itertools::Itertools;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Cell {
    Empty,
    Full,
}

struct Grid(Vec<Vec<Cell>>);

impl Grid {
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|x| match x {
                        b'.' => Cell::Empty,
                        b'@' => Cell::Full,
                        _ => panic!(),
                    })
                    .collect_vec()
            })
            .collect_vec();
        Grid(grid)
    }

    fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.0.get(y)?.get(x)
    }
    fn dims(&self) -> (usize, usize) {
        (self.0[0].len(), self.0.len())
    }

    fn count_full_neighbors(&self, x: usize, y: usize) -> usize {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|&(i, j)| i != 0 || j != 0)
            .filter(|&(i, j)| self.get(x + i as usize, y + j as usize) == Some(&Cell::Full))
            .count()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let grid = Grid::new(&input);
    let dims = grid.dims();
    let result = (0..dims.0)
        .cartesian_product(0..dims.1)
        .filter(|&(x, y)| grid.get(x, y) == Some(&Cell::Full))
        .filter(|&(x, y)| grid.count_full_neighbors(x, y) < 4)
        .count();
    println!("{}", result);
    Ok(())
}
