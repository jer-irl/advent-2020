use std::collections::HashSet;

use crate::errors::AdventError;

pub fn solve(input: &str) -> Result<(), AdventError> {
    let mut space = TupleHashSetSparseSpace::new(input);
    for _ in 0..6 {
        space.simulate_step();
    }
    let result = space.num_items();
    println!("{:?}", result);

    Ok(())
}

trait SparseSpace {
    fn num_items(&self) -> usize;
    fn simulate_step(&mut self);
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    layer: isize,
    row: isize,
    col: isize,
}

impl Point {
    fn from_lrc(layer: isize, row: isize, col: isize) -> Self {
        Self { layer, row, col }
    }
}

struct TupleHashSetSparseSpace {
    data: HashSet<Point>,
}

impl TupleHashSetSparseSpace {
    pub fn new(input: &str) -> Self {
        let mut data = HashSet::new();
        for (row_idx, row) in input.lines().enumerate() {
            for (col_idx, c) in row.bytes().enumerate() {
                if c == b'#' {
                    data.insert(Point::from_lrc(0, row_idx as isize, col_idx as isize));
                }
            }
        }
        Self { data }
    }
}

impl SparseSpace for TupleHashSetSparseSpace {
    fn num_items(&self) -> usize {
        self.data.len()
    }

    fn simulate_step(&mut self) {
        let mut new_data = HashSet::new();

        let (mut min_l, mut min_r, mut min_c) = (isize::max_value(), isize::max_value(), isize::max_value());
        let (mut max_l, mut max_r, mut max_c) = (isize::min_value(), isize::min_value(), isize::min_value());
        for Point {layer, row, col} in self.data.iter() {
            min_l = min_l.min(*layer);
            min_r = min_r.min(*row);
            min_c = min_c.min(*col);
            max_l = max_l.max(*layer);
            max_r = max_r.max(*row);
            max_c = max_c.max(*col);
        }

        min_l -= 1;
        min_r -= 1;
        min_c -= 1;
        max_l += 1;
        max_r += 1;
        max_c += 1;

        for layer in min_l..=max_l {
            for row in min_r..=max_r {
                for col in min_c..=max_c {
                    let num_neighbors = {
                        let mut result = 0;
                        for l in (layer - 1)..=(layer + 1) {
                            for r in (row - 1)..=(row + 1) {
                                for c in (col - 1)..=(col + 1) {
                                    if l != layer || r != row || c != col {
                                        if self.data.contains(&Point { layer: l, row: r, col: c }) {
                                            result += 1;
                                        }
                                    }
                                }
                            }
                        }
                        result
                    };

                    if self.data.contains(&Point { layer, row, col }) {
                        if num_neighbors == 2 || num_neighbors == 3 {
                            new_data.insert(Point { layer, row, col });
                        } else {
                            continue;
                        }
                    } else {
                        if num_neighbors == 3 {
                            new_data.insert(Point { layer, row, col });
                        } else {
                            continue;
                        }
                    }
                }
            }
        }
        self.data = new_data;
    }
}
