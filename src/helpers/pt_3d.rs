use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;
use std::sync::OnceLock;

use anyhow::{anyhow, Context};
use num::{zero, Signed, Zero};
use regex::Regex;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pt3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Pt3d<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T, U, V, W> From<(U, V, W)> for Pt3d<T>
where
    U: Into<T>,
    V: Into<T>,
    W: Into<T>,
{
    fn from(value: (U, V, W)) -> Self {
        Self::new(value.0.into(), value.1.into(), value.2.into())
    }
}

impl<T, U, V, W> From<Pt3d<T>> for (U, V, W)
where
    T: Into<U> + Into<V> + Into<W>,
{
    fn from(value: Pt3d<T>) -> Self {
        (value.x.into(), value.y.into(), value.z.into())
    }
}

impl<T> FromStr for Pt3d<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let re = REGEX.get_or_init(|| {
            Regex::new(
                r"\(?(?<x>-?\d+(?:\.\d*)?),\s*(?<y>-?\d+(?:\.\d*)?),\s*(?<z>-?\d+(?:\.\d*)?)\)?$",
            )
            .unwrap()
        });

        let captures = re.captures(s).ok_or(anyhow!("wrong Pt3d format: {}", s))?;
        let get_capture = |name: &str| {
            captures[name]
                .parse::<T>()
                .with_context(|| format!("invalid {} value: {}", name, &captures[name]))
        };

        Ok(Self::new(get_capture("x")?, get_capture("y")?, get_capture("z")?))
    }
}

impl<T> Display for Pt3d<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T> Add for Pt3d<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> AddAssign for Pt3d<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> Sub for Pt3d<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> SubAssign for Pt3d<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T> Zero for Pt3d<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Self::new(zero(), zero(), zero())
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}

pub fn manhattan<T>(a: Pt3d<T>, b: Pt3d<T>) -> T
where
    T: Signed,
{
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
}