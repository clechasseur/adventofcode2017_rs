use std::ops::Mul;

use crate::helpers::knot_hash::KnotHash;
use crate::input::day_10::INPUT;

pub fn part_1() -> usize {
    KnotHash::sparse(part_1_lengths(), 1)
        .into_iter()
        .map(|n| n as usize)
        .take(2)
        .reduce(Mul::mul)
        .unwrap()
}

pub fn part_2() -> String {
    KnotHash::new(INPUT).to_string()
}

fn part_1_lengths() -> Vec<u8> {
    INPUT
        .split(',')
        .map(|length| length.parse().unwrap())
        .collect()
}
