use crate::Neighborhood::{Owned, View};
use nalgebra::iter::MatrixIter;
use nalgebra::{Const, DMatrix, Dyn, MatrixView, OMatrix, VecStorage, ViewStorage};
use std::fmt::Debug;

pub mod renderer;
pub mod rulesets;
#[macro_use]
pub use rulesets::*;
pub type CellStateType = u8;

pub enum Neighborhood<'a> {
    View(MatrixView<'a, CellStateType, Dyn, Dyn>),
    Owned(OMatrix<CellStateType, Dyn, Dyn>),
}

pub enum NeighborhoodIter<'a> {
    View(MatrixIter<'a, CellStateType, Dyn, Dyn, ViewStorage<'a, CellStateType, Dyn, Dyn, Const<1>, Dyn>>),
    Owned(MatrixIter<'a, CellStateType, Dyn, Dyn, VecStorage<CellStateType, Dyn, Dyn>>),
}

impl Neighborhood<'_> {
    pub fn iter(&self) -> NeighborhoodIter<'_> {
        match self {
            Owned(matrix) => NeighborhoodIter::Owned(matrix.iter()),
            View(matrix) => NeighborhoodIter::View(matrix.iter()),
        }
    }

    pub fn get(&self, index: (usize, usize)) -> Option<&CellStateType> {
        match self {
            Owned(matrix) => matrix.get(index),
            View(matrix) => matrix.get(index),
        }
    }

    pub fn nrows(&self) -> usize {
        match self {
            View(matrix) => matrix.nrows(),
            Owned(matrix) => matrix.nrows(),
        }
    }

    pub fn ncols(&self) -> usize {
        match self {
            View(matrix) => matrix.ncols(),
            Owned(matrix) => matrix.ncols(),
        }
    }
}

impl<'a> Iterator for NeighborhoodIter<'a> {
    type Item = &'a CellStateType;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            NeighborhoodIter::Owned(iterator) => iterator.next(),
            NeighborhoodIter::View(iter) => iter.next(),
        }
    }
}

// pub trait Rule {
//     fn apply_to_neighborhood(n :&Neighborhood, s: &CellStateType) -> Option<CellStateType>;
// }
pub type Rule = fn(&Neighborhood, &CellStateType) -> Option<CellStateType>;

#[derive(Debug)]
pub enum RetrievalMode {
    Wrapping,
    Padded,
}

#[derive(Debug)]
pub struct Engine {
    pub grid: OMatrix<CellStateType, Dyn, Dyn>,
    pub rules: Vec<Rule>,
    neighbourhood_shape: (usize, usize),
    pub retrieval_mode: RetrievalMode,
    neighbourhood_indices: [OMatrix<i32, Dyn, Dyn>; 2],
    pub paused: bool,
}

impl Engine {
    pub fn new(
        grid: OMatrix<CellStateType, Dyn, Dyn>,
        rules: Vec<Rule>,
        neighbourhood_shape: (usize, usize),
        retrieval_mode: RetrievalMode,
    ) -> Result<Engine, String> {
        if grid.nrows() <= neighbourhood_shape.0 || grid.ncols() <= neighbourhood_shape.1 {
            return Err("neighbourhood shape must be strictly smaller than the grid shape".to_string());
        }

        let mut row_indices: OMatrix<i32, Dyn, Dyn> = DMatrix::zeros(neighbourhood_shape.0, neighbourhood_shape.1);
        let mut col_indices = row_indices.clone_owned();
        (0..neighbourhood_shape.0).for_each(|i| row_indices.fill_row(i, i as i32));
        (0..neighbourhood_shape.1).for_each(|i| col_indices.fill_column(i, i as i32));
        row_indices = row_indices.add_scalar(-(neighbourhood_shape.0 as i32 / 2));
        col_indices = col_indices.add_scalar(-(neighbourhood_shape.1 as i32 / 2));

        Ok(Engine {
            grid,
            rules,
            neighbourhood_shape,
            retrieval_mode,
            neighbourhood_indices: [row_indices, col_indices],
            paused: false,
        })
    }

    pub fn get_neighbourhood(&self, index: (usize, usize)) -> Neighborhood {
        self._get_neighbourhood_from_view(index)
            .unwrap_or(self._get_neighbourhood_from_indices(index))
    }

    fn _get_neighbourhood_from_view(&self, index: (usize, usize)) -> Option<Neighborhood> {
        let (half_width, half_height) = (self.neighbourhood_shape.0 / 2, self.neighbourhood_shape.1 / 2);
        if index.0 + half_width >= self.grid.ncols() || index.1 + half_height >= self.grid.nrows() {
            return None;
        }

        let start_index = (index.0.checked_sub(half_width)?, index.1.checked_sub(half_height)?);

        Some(Neighborhood::View(
            self.grid.view(start_index, self.neighbourhood_shape),
        ))
    }

    fn _get_neighbourhood_from_indices(&self, index: (usize, usize)) -> Neighborhood {
        let (row_indices, col_indices) = (
            self.neighbourhood_indices[0].add_scalar(index.0 as i32),
            self.neighbourhood_indices[1].add_scalar(index.1 as i32),
        );

        let mut ret: OMatrix<CellStateType, Dyn, Dyn> =
            DMatrix::zeros(self.neighbourhood_shape.0, self.neighbourhood_shape.1);
        for (row, (i, column)) in row_indices.iter().zip(col_indices.iter().enumerate()) {
            *ret.index_mut(i) = self._get_from_grid(*row, *column);
        }
        Neighborhood::Owned(ret)
    }

    fn _get_from_grid(&self, mut row: i32, mut col: i32) -> CellStateType {
        let (nrows, ncols) = (self.grid.nrows(), self.grid.ncols());

        match self.retrieval_mode {
            RetrievalMode::Padded => {
                if row < 0 || row > nrows as i32 || col < 0 || col > ncols as i32 {
                    return 0 as CellStateType;
                }
            }
            RetrievalMode::Wrapping => {
                row = if row < 0 {
                    nrows as i32 + row
                } else {
                    row % nrows as i32
                };
                col = if col < 0 {
                    ncols as i32 + col
                } else {
                    col % ncols as i32
                };
            }
        }
        self.grid[(row as usize, col as usize)]
    }

    pub fn step(&mut self) {
        if self.paused {
            return;
        }

        let mut ret = self.grid.clone();
        for i in 0..self.grid.ncols() {
            for j in 0..self.grid.nrows() {
                let neighbourhood = self.get_neighbourhood((i, j));
                for rule in &self.rules {
                    match rule(&neighbourhood, &self.grid[(i, j)]) {
                        None => {}
                        Some(val) => {
                            ret[(i, j)] = val;
                            break;
                        }
                    }
                    ret[(i, j)] = self.grid[(i, j)];
                }
            }
        }
        self.grid = ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tets_wrapping_neighborhood() -> Result<(), String> {
        let grid = DMatrix::from_fn(9, 9, |row, col| (row * 10 + col) as CellStateType);
        let engine = Engine::new(grid, vec![], (3, 3).into(), RetrievalMode::Wrapping)?;

        let n = engine.get_neighbourhood((0, 0));
        for i in 0..3 {
            for j in 0..3 {
                let expected: u8 = ((i + 8) % 9) * 10 + (j + 8) % 9;
                println!("i: {}, j: {}, expected: {}", i, j, expected);
                assert_eq!(*(n.get((i as usize, j as usize)).expect("")), expected);
            }
        }

        let n = engine.get_neighbourhood((8, 8));
        for i in 0..3 {
            for j in 0..3 {
                let expected: u8 = ((i + 7) % 9) * 10 + (j + 7) % 9;
                let actual: CellStateType = *(n.get((i as usize, j as usize)).expect(""));
                println!("i: {}, j: {}, expected: {}, actual: {}", i, j, expected, actual);
                assert_eq!(actual, expected);
            }
        }
        Ok(())
    }
}
