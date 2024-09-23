use std::fmt::{Display, Formatter};
use std::ops::{Add, Neg};

use num::{zero, One, Zero};

use crate::helpers::direction::Direction;
use crate::helpers::pt::Pt;

/// A [turtle] moving around 2D space.
///
/// [turtle]: https://en.wikipedia.org/wiki/Logo_(programming_language)#Turtle_and_graphics
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Turtle<T = i64> {
    pub position: Pt<T>,
    pub direction: Direction,
}

impl<T> Turtle<T> {
    /// Returns a new [`Turtle`] starting from the given position, facing the given direction.
    pub fn new(position: Pt<T>, direction: Direction) -> Self {
        Self { position, direction }
    }

    /// Returns a new [`Turtle`] starting at point (0, 0) facing the given direction.
    pub fn from_zero(direction: Direction) -> Self
    where
        Pt<T>: Zero,
    {
        Self::new(zero(), direction)
    }

    /// Turns the [`Turtle`] 90 degrees to the left.
    pub fn turn_left(&self) -> Self
    where
        Pt<T>: Copy,
    {
        Self { direction: self.direction.turn_left(), ..*self }
    }

    /// Turns the [`Turtle`] 90 degrees to the right.
    pub fn turn_right(&self) -> Self
    where
        Pt<T>: Copy,
    {
        Self { direction: self.direction.turn_right(), ..*self }
    }

    /// Turns the [`Turtle`] around 180 degrees.
    pub fn turn_around(&self) -> Self
    where
        Pt<T>: Copy,
    {
        Self { direction: self.direction.turn_around(), ..*self }
    }

    /// Advances the [`Turtle`] one step in the direction it is currently facing.
    pub fn advance(&self) -> Self
    where
        T: Zero + One + Neg<Output = T> + Add<Output = T>,
        Pt<T>: Copy,
    {
        Self { position: self.position + self.direction.displacement(), ..*self }
    }
}

impl<T> Display for Turtle<T>
where
    Pt<T>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ position: {}, direction: {} }}", self.position, self.direction)
    }
}
