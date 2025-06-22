use crate::{CellStateType, Neighborhood, Rule};

pub fn game_of_life() -> Vec<Rule> {
    vec![
        |n: &Neighborhood, s: &CellStateType| {
            if n.iter().sum::<CellStateType>() - s < 2 {
                Some(0)
            } else {
                None
            }
        },
        |n: &Neighborhood, s: &CellStateType| {
            if n.iter().sum::<CellStateType>() - s == 3 {
                Some(1)
            } else {
                None
            }
        },
        |n: &Neighborhood, s: &CellStateType| {
            if n.iter().sum::<CellStateType>() - s > 3 {
                Some(0)
            } else {
                None
            }
        },
    ]
}

#[macro_export]
macro_rules! maze_generation {
    ($activate_at:expr, $limits:expr) => {
        vec![
            |n: &$crate::Neighborhood, s: &$crate::CellStateType| {
                let sum = n.iter().sum::<$crate::CellStateType>() - s;
                if $activate_at.contains(&sum) { Some(1) } else { None }
            },
            |n: &$crate::Neighborhood, s: &$crate::CellStateType| {
                let sum = n.iter().sum::<$crate::CellStateType>() - s;
                if sum < $limits.0 || sum > $limits.1 {
                    Some(0)
                } else {
                    None
                }
            },
        ]
    };
}
