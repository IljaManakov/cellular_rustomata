use nalgebra::{DMatrix, Dyn, MatrixView, OMatrix};

// #[derive(Debug)]
// pub struct Cell{
//     pub state: Vec<f32>,
//     pub position: Vec<i32>,
//     pub neighbors: Vec<Cell>
// }

type Neighbourhood<'a> = MatrixView<'a, f32, Dyn, Dyn>;
type Rule = fn(&Neighbourhood) -> Option<f32>;

#[derive(Debug)]
pub enum RetrievalMode {
    Wrapping,
    Padded,
}

#[derive(Debug)]
pub struct Engine {
    pub grid: OMatrix<f32, Dyn, Dyn>,
    pub rules: Vec<Rule>,
    neighbourhood_shape: [usize; 2],
    pub retrieval_mode: RetrievalMode,
    neighbourhood_indices: [OMatrix<i32, Dyn, Dyn>; 2],
}


impl Engine {
    ///
    ///
    /// # Arguments
    ///
    /// * `grid`:
    /// * `rules`:
    /// * `neighbourhood_shape`:
    /// * `retrieval_mode`:
    ///
    /// returns: Result<Engine, String>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn new(grid: OMatrix<f32, Dyn, Dyn>,
               rules: Vec<Rule>,
               neighbourhood_shape: [usize; 2],
               retrieval_mode: RetrievalMode) -> Result<Engine, String> {
        if grid.nrows() <= neighbourhood_shape[0] || grid.ncols() <= neighbourhood_shape[1] {
            return Err("neighbourhood shape must be strictly smaller than the grid shape".to_string());
        }

        let mut row_indices: OMatrix<i32, Dyn, Dyn> = DMatrix::zeros(neighbourhood_shape[0], neighbourhood_shape[1]);
        let mut col_indices = row_indices.clone_owned();
        (0..neighbourhood_shape[0]).for_each(|i| row_indices.fill_row(i, i as i32));
        (0..neighbourhood_shape[1]).for_each(|i| col_indices.fill_column(i, i as i32));
        row_indices = row_indices.add_scalar(-(neighbourhood_shape[0] as i32 / 2));
        col_indices = col_indices.add_scalar(-(neighbourhood_shape[1] as i32 / 2));

        Ok(Engine { grid, rules, neighbourhood_shape, retrieval_mode, neighbourhood_indices: [row_indices, col_indices] })
    }

    pub fn get_neighbourhood(&self, index: &[usize; 2]) -> OMatrix<f32, Dyn, Dyn> {
        self._get_neighbourhood_from_view(index).unwrap_or(self._get_neighbourhood_from_indices(index))
    }

    fn _get_neighbourhood_from_view(&self, index: &[usize; 2]) -> Option<OMatrix<f32, Dyn, Dyn>> {
        let (half_width, half_height) = (self.neighbourhood_shape[0] / 2, self.neighbourhood_shape[1] / 2);
        if index[0] + half_width > self.grid.ncols() || index[0] + half_width > self.grid.nrows() {
            return None;
        }

        let start = index
            .iter()
            .zip([half_width, half_height].iter())
            .map(|(&ind, &extent)| ind.checked_sub(extent))
            .collect::<Option<Vec<usize>>>()?;

        Some(self.grid.view((start[0], start[1]), self.neighbourhood_shape.into()).clone_owned())
    }

    fn _get_neighbourhood_from_indices(&self, index: &[usize; 2]) -> OMatrix<f32, Dyn, Dyn> {
        let (row_indices, col_indices) = (
            self.neighbourhood_indices[0].add_scalar(index[0] as i32),
            self.neighbourhood_indices[1].add_scalar(index[1] as i32)
        );

        let mut ret: OMatrix<f32, Dyn, Dyn> = DMatrix::zeros(self.neighbourhood_shape[0], self.neighbourhood_shape[1]);
        for (row, (i, column)) in row_indices.iter().zip(col_indices.iter().enumerate()) {
            *ret.index_mut(i) = self._get_from_grid(*row, *column);
        }
        ret
    }

    fn _get_from_grid(&self, mut row: i32, mut col: i32) -> f32 {
        let (nrows, ncols) = (self.grid.nrows(), self.grid.ncols());

        match self.retrieval_mode {
            RetrievalMode::Padded => if row < 0 || row > nrows as i32 || col < 0 || col > ncols as i32 { return 0f32; }
            RetrievalMode::Wrapping => {
                row = if row < 0 { nrows as i32 + row } else { row % nrows as i32 };
                col = if col < 0 { ncols as i32 + col } else { col % ncols as i32 };
            }
        }
        self.grid[(row as usize, col as usize)]
    }
}

impl Iterator for Engine {
    type Item = OMatrix<f32, Dyn, Dyn>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.grid.clone();
        Some(ret.add_scalar(1.0))
    }
}
