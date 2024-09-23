use std::iter::successors;

use itertools::Itertools;
use num::{zero, Zero};
use strum::EnumString;

use crate::helpers::pt::{manhattan, Pt};
use crate::input::day_11::INPUT;

pub fn part_1() -> usize {
    distance_to(child_position())
}

pub fn part_2() -> usize {
    child_path()
        .sorted_by_key(|&pt| -manhattan(Pt::zero(), pt))
        .map(distance_to)
        .next()
        .unwrap()
}

fn child_path() -> impl Iterator<Item = Pt> {
    INPUT
        .split(',')
        .map(|dir| dir.parse::<HexDirection>().unwrap())
        .scan(zero(), |pt: &mut Pt, dir| {
            *pt += dir.displacement();
            Some(*pt)
        })
}

fn child_position() -> Pt {
    child_path().last().unwrap()
}

fn distance_to(goal: Pt) -> usize {
    path_to(goal).count() - 1
}

fn path_to(goal: Pt) -> impl Iterator<Item = Pt> {
    successors(Some(zero()), move |&pt: &Pt| {
        match ((goal.x - pt.x).signum(), (goal.y - pt.y).signum()) {
            (0, 0) => None,
            (0, y) => Some(pt + Pt::new(0, y * 2)),
            (x, y) => Some(pt + Pt::new(x, y)),
        }
    })
}

#[derive(Debug, Copy, Clone, EnumString)]
#[strum(serialize_all = "snake_case")]
enum HexDirection {
    NW,
    N,
    NE,
    SE,
    S,
    SW,
}

impl HexDirection {
    fn displacement(&self) -> Pt {
        match self {
            HexDirection::NW => Pt::new(-1, 1),
            HexDirection::N => Pt::new(0, 2),
            HexDirection::NE => Pt::new(1, 1),
            HexDirection::SE => Pt::new(1, -1),
            HexDirection::S => Pt::new(0, -2),
            HexDirection::SW => Pt::new(-1, -1),
        }
    }
}
