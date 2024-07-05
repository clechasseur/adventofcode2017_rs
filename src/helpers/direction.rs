use std::str::FromStr;

use strum::{EnumCount, EnumProperty, FromRepr};

use crate::helpers::pt::Pt;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, FromRepr, EnumCount, EnumProperty)]
pub enum Direction {
    #[strum(props(displacement = "(1, 0)"))]
    Right,
    #[strum(props(displacement = "(0, -1)"))]
    Down,
    #[strum(props(displacement = "(-1, 0)"))]
    Left,
    #[strum(props(displacement = "(0, 1)"))]
    Up,
}

impl Direction {
    pub fn turn_left(&self) -> Self {
        Self::from_repr(((*self as u8) + 3) % (Self::COUNT as u8)).unwrap()
    }

    pub fn turn_right(&self) -> Self {
        Self::from_repr(((*self as u8) + 1) % (Self::COUNT as u8)).unwrap()
    }

    pub fn displacement<T>(&self) -> Pt<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {
        self.get_str("displacement").unwrap().parse().unwrap()
    }
}
