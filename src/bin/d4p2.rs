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
        self.0.get(x)?.get(y)
    }
    fn dims(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }

    fn removable(&self, x: usize, y: usize) -> bool {
        self.0[x][y] == Cell::Full
            && (-1..=1)
                .cartesian_product(-1..=1)
                .filter(|&(i, j)| i != 0 || j != 0)
                .filter(|&(i, j)| self.get(x + i as usize, y + j as usize) == Some(&Cell::Full))
                .count()
                < 4
    }

    fn remove(&mut self, x: usize, y: usize) -> bool {
        if self.removable(x, y) {
            self.0[x][y] = Cell::Empty;
            true
        } else {
            false
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut grid = Grid::new(&input);
    let dims = grid.dims();
    let mut result = 0;
    let mut any_removable = true;
    while any_removable {
        any_removable = false;
        for x in 0..dims.0 {
            for y in 0..dims.1 {
                if grid.remove(x, y) {
                    result += 1;
                    any_removable = true;
                }
            }
        }
    }
    println!("{}", result);
    Ok(())
}
