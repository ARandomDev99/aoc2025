use itertools::Itertools;
use std::collections::BinaryHeap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Vertex(u128, u128, u128);

impl Vertex {
    fn distance(lhs: Self, rhs: Self) -> u128 {
        (lhs.0 - rhs.0).pow(2) + (lhs.1 - rhs.1).pow(2) + (lhs.2 - rhs.2).pow(2)
    }
}

struct Edge {
    from: usize,
    to: usize,
    distance: u128,
}

impl std::cmp::PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl std::cmp::Eq for Edge {}

impl std::cmp::PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse comparison to for min-heap
        other.distance.cmp(&self.distance)
    }
}

struct Space {
    vertices: Vec<Vertex>,
    min_heap: BinaryHeap<Edge>,
    edges: Vec<Vec<bool>>,
}

impl Space {
    fn new(input: String) -> Self {
        let vertices = input
            .lines()
            .map(|line| {
                let (x, y, z) = line
                    .split(',')
                    .map(|part| part.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                Vertex(x, y, z)
            })
            .collect_vec();
        assert!(vertices.iter().all_unique());
        let mut min_heap = BinaryHeap::new();
        for i in 0..vertices.len() {
            for j in i + 1..vertices.len() {
                min_heap.push(Edge {
                    from: i,
                    to: j,
                    distance: Vertex::distance(vertices[i], vertices[j]),
                });
            }
        }
        let edges = vec![vec![false; vertices.len()]; vertices.len()];
        Space {
            vertices,
            min_heap,
            edges,
        }
    }
    fn add_edge(&mut self, i: usize, j: usize) {
        self.edges[i][j] = true;
        self.edges[j][i] = true;
    }
}

impl std::fmt::Debug for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.edges {
            for edge in row {
                write!(f, "{} ", edge)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut graph = Space::new(input);
    let mut regions = (0..graph.vertices.len()).collect_vec();
    let mut prev = (0, 0);
    while regions.iter().unique().count() > 1 {
        let Some(Edge { from: i, to: j, .. }) = graph.min_heap.pop() else {
            break;
        };
        prev = (i, j);
        graph.add_edge(i, j);
        regions = (0..graph.vertices.len()).collect_vec();
        for i in 0..graph.vertices.len() {
            for j in (i + 1..graph.vertices.len()).filter(|&j| graph.edges[i][j]) {
                let new_region = regions[i];
                let old_region = regions[j];
                regions.iter_mut().for_each(|region| {
                    if *region == old_region {
                        *region = new_region;
                    }
                });
            }
        }
    }
    println!("{}", graph.vertices[prev.0].0 * graph.vertices[prev.1].0);
    Ok(())
}
