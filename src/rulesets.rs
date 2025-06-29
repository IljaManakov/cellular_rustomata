use crate::{CellStateType, Neighborhood, Rules};

#[derive(Debug)]
pub struct LifeLike {
    birth: Vec<CellStateType>,
    survival: Vec<CellStateType>,
}

impl LifeLike {
    pub fn new(birth: Vec<CellStateType>, survival: Vec<CellStateType>) -> Self {
        LifeLike { birth, survival }
    }
}

impl Rules for LifeLike {
    fn step(&self, neighborhood: &Neighborhood, current_state: CellStateType) -> CellStateType {
        let sum = neighborhood.iter().sum::<CellStateType>() - current_state;
        if self.birth.contains(&sum) {
            return 1 as CellStateType;
        }
        if self.survival.contains(&sum) {
            return current_state;
        }
        0 as CellStateType
    }
}

#[derive(Debug)]
pub struct GameOfLife {
    life_like: LifeLike,
}

impl GameOfLife {
    pub fn new() -> Self {
        GameOfLife {
            life_like: LifeLike::new(vec![3], vec![2, 3]),
        }
    }
}

impl Rules for GameOfLife {
    fn step(&self, neighborhood: &Neighborhood, current_state: CellStateType) -> CellStateType {
        self.life_like.step(neighborhood, current_state)
    }
}

#[derive(Debug)]
pub struct Maze {
    life_like: LifeLike,
}

impl Maze {
    pub fn new() -> Self {
        Maze {
            life_like: LifeLike::new(vec![3], vec![1, 2, 3, 4, 5]),
        }
    }
}

impl Rules for Maze {
    fn step(&self, neighborhood: &Neighborhood, current_state: CellStateType) -> CellStateType {
        self.life_like.step(neighborhood, current_state)
    }
}

#[derive(Debug)]
pub struct Mazectric {
    life_like: LifeLike,
}

impl Mazectric {
    pub fn new() -> Self {
        Mazectric {
            life_like: LifeLike::new(vec![3], vec![1, 2, 3, 4]),
        }
    }
}

impl Rules for Mazectric {
    fn step(&self, neighborhood: &Neighborhood, current_state: CellStateType) -> CellStateType {
        self.life_like.step(neighborhood, current_state)
    }
}
