use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::iter::successors;
use std::str::FromStr;

use itertools::Itertools;

use crate::input::day_21::INPUT;

pub fn part_1() -> usize {
    on_count_after(5)
}

pub fn part_2() -> usize {
    on_count_after(18)
}

fn on_count_after(iterations: usize) -> usize {
    iterate(Rules::default())
        .nth(iterations)
        .unwrap()
        .on_count()
}

const INITIAL_PATTERN: &str = ".#./..#/###";

fn iterate(rules: Rules) -> impl Iterator<Item = Pattern> {
    let pattern: Pattern = INITIAL_PATTERN.parse().unwrap();

    successors(Some(pattern), move |pattern| Some(pattern.enhance(&rules)))
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Pattern(Vec<String>);

impl Pattern {
    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn on_count(&self) -> usize {
        self.0
            .iter()
            .map(|line| line.bytes().filter(|&b| b == b'#').count())
            .sum()
    }

    pub fn enhance(&self, rules: &Rules) -> Self {
        let chunk_size = if self.size() % 2 == 0 { 2 } else { 3 };

        Self(
            self.0
                .chunks(chunk_size)
                .flat_map(|lines_chunk| {
                    let enhanced_line = (0..self.size() / chunk_size)
                        .map(move |chunk_idx| {
                            let start = chunk_idx * chunk_size;

                            Self(
                                lines_chunk
                                    .iter()
                                    .map(|line| line[start..start + chunk_size].to_string())
                                    .collect_vec(),
                            )
                        })
                        .map(|pattern| rules.transform(&pattern))
                        .collect_vec();

                    (0..=chunk_size).map(move |line_idx| {
                        enhanced_line
                            .iter()
                            .map(|pattern| &pattern.0[line_idx])
                            .join("")
                    })
                })
                .collect_vec(),
        )
    }

    pub fn into_combinations(self) -> impl Iterator<Item = Self> {
        vec![self.flip_horizontally(), self]
            .into_iter()
            .flat_map(|pattern| vec![pattern.flip_vertically(), pattern])
            .flat_map(Self::rotations)
    }

    fn flip_horizontally(&self) -> Self {
        Self(self.0.iter().rev().cloned().collect_vec())
    }

    fn flip_vertically(&self) -> Self {
        Self(
            self.0
                .iter()
                .map(|line| String::from_utf8(line.bytes().rev().collect_vec()).unwrap())
                .collect_vec(),
        )
    }

    fn columns(&self) -> impl DoubleEndedIterator<Item = String> + '_ {
        (0..self.size()).map(|col_idx| {
            String::from_utf8(
                self.0
                    .iter()
                    .map(|line| line.as_bytes()[col_idx])
                    .collect_vec(),
            )
            .unwrap()
        })
    }

    fn rotate_left(&self) -> Self {
        Self(self.columns().rev().collect_vec())
    }

    fn rotations(self) -> impl Iterator<Item = Self> {
        successors(Some(self), |pattern| Some(pattern.rotate_left())).take(4)
    }
}

impl FromStr for Pattern {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split("/").map(Into::into).collect_vec()))
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sep = if f.alternate() { "\n" } else { "/" };
        write!(f, "{}", self.0.join(sep))
    }
}

#[derive(Debug, Clone)]
struct Rules(HashMap<Pattern, Pattern>);

impl Rules {
    pub fn transform(&self, pattern: &Pattern) -> Pattern {
        self.0
            .get(pattern)
            .unwrap_or_else(|| panic!("no rule found for '{pattern}'"))
            .clone()
    }
}

impl FromStr for Rules {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .flat_map(|line| {
                    let (from, to) = line
                        .split(" => ")
                        .map(|pat| pat.parse::<Pattern>().unwrap())
                        .collect_tuple()
                        .unwrap();

                    from.into_combinations()
                        .map(move |pattern| (pattern, to.clone()))
                })
                .collect(),
        ))
    }
}

impl Default for Rules {
    fn default() -> Self {
        INPUT.parse().unwrap()
    }
}
