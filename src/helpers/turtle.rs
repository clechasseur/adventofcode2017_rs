use std::fmt::{Display, Formatter};
use std::ops::{Add, Neg};

use num::{One, Zero};

use crate::helpers::direction::Direction;
use crate::helpers::pt::Pt;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Turtle<T> {
    pos: Pt<T>,
    dir: Direction,
}

impl<T> Turtle<T> {
    pub fn new(pos: Pt<T>, dir: Direction) -> Self {
        Self { pos, dir }
    }

    pub fn direction(&self) -> Direction {
        self.dir
    }
}

impl<T> Turtle<T>
where
    Pt<T>: Copy,
{
    pub fn position(&self) -> Pt<T> {
        self.pos
    }
}

impl<T> Turtle<T>
where
    T: Zero,
{
    pub fn at_center(dir: Direction) -> Self {
        Self::new(Pt::zero(), dir)
    }
}

impl<T> Turtle<T>
where
    T: Copy,
{
    pub fn turn_left(&self) -> Self {
        Self { pos: self.pos, dir: self.dir.turn_left() }
    }

    pub fn turn_right(&self) -> Self {
        Self { pos: self.pos, dir: self.dir.turn_right() }
    }
}

impl<T> Turtle<T>
where
    T: Copy + Zero + One + Neg<Output = T> + Add<Output = T>,
{
    pub fn advance(&self) -> Self {
        Self { pos: self.pos + self.dir.displacement(), dir: self.dir }
    }
}

impl<T> Display for Turtle<T>
where
    Pt<T>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ position: {}, direction: {} }}", self.pos, self.dir)
    }
}
