use std::ops::Neg;

use num::{one, zero, One, Zero};
use strum::{Display, EnumCount, FromRepr};

use crate::helpers::pt::Pt;

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, FromRepr, EnumCount, Display,
)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    pub fn turn_left(&self) -> Self {
        Self::from_repr(((*self as u8) + 3) % (Self::COUNT as u8)).unwrap()
    }

    pub fn turn_right(&self) -> Self {
        Self::from_repr(((*self as u8) + 1) % (Self::COUNT as u8)).unwrap()
    }

    pub fn turn_around(&self) -> Self {
        Self::from_repr(((*self as u8) + 2) % (Self::COUNT as u8)).unwrap()
    }

    pub fn displacement<T>(&self) -> Pt<T>
    where
        T: Zero + One + Neg<Output = T>,
    {
        match self {
            Direction::Right => Pt::new(one(), zero()),
            Direction::Down => Pt::new(zero(), -one::<T>()),
            Direction::Left => Pt::new(-one::<T>(), zero()),
            Direction::Up => Pt::new(zero(), one()),
        }
    }
}
