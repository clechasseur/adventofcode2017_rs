use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;
use std::sync::OnceLock;

use num::{zero, Signed, Zero};
use regex::Regex;

use crate::helpers::regex::CapturesHelper;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pt<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pt<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T, U, V> From<(U, V)> for Pt<T>
where
    U: Into<T>,
    V: Into<T>,
{
    fn from(value: (U, V)) -> Self {
        Self::new(value.0.into(), value.1.into())
    }
}

impl<T, U, V> From<Pt<T>> for (U, V)
where
    T: Into<U> + Into<V>,
{
    fn from(value: Pt<T>) -> Self {
        (value.x.into(), value.y.into())
    }
}

impl<T> FromStr for Pt<T>
where
    T: FromStr,
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let re = REGEX.get_or_init(|| {
            Regex::new(r"\(?(?<x>-?\d+(?:\.\d*)?),\s*(?<y>-?\d+(?:\.\d*)?)\)?$").unwrap()
        });

        let captures = re
            .captures(s)
            .unwrap_or_else(|| panic!("invalid Pt value: {s}"));
        Ok(Self::new(captures.ez_get("x"), captures.ez_get("y")))
    }
}

impl<T> Display for Pt<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> Add for Pt<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> AddAssign for Pt<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for Pt<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> SubAssign for Pt<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Zero for Pt<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Self::new(zero(), zero())
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

pub fn manhattan<T>(a: Pt<T>, b: Pt<T>) -> T
where
    T: Signed,
{
    (a.x - b.x).abs() + (a.y - b.y).abs()
}
