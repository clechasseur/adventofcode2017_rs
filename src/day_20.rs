use std::iter::successors;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::OnceLock;

use itertools::Itertools;
use num::zero;
use regex::Regex;

use crate::helpers::pt_3d::{manhattan, Pt3d};
use crate::helpers::regex::CapturesHelper;
use crate::input::day_20::INPUT;

pub fn part_1() -> usize {
    Universe::default()
        .iter()
        .sorted_unstable_by(|p1, p2| {
            distance_to_0(p1.acceleration)
                .cmp(&distance_to_0(p2.acceleration))
                .then_with(|| distance_to_0(p1.velocity).cmp(&distance_to_0(p2.velocity)))
                .then_with(|| distance_to_0(p1.position).cmp(&distance_to_0(p2.position)))
        })
        .next()
        .unwrap()
        .id
}

pub fn part_2() -> usize {
    expanding_universe().last().unwrap().len()
}

fn expanding_universe() -> impl Iterator<Item = Universe> {
    successors(Some(Universe::default()), |universe| {
        let expanded = universe.move_one_tick();

        let changing = universe
            .particle_ids()
            .zip(expanded.particle_ids())
            .any(|(p1, p2)| p1 != p2);
        changing.then_some(expanded)
    })
}

type Coords = Pt3d<i64>;

fn distance_to_0(c: Coords) -> i64 {
    manhattan(zero(), c)
}

#[derive(Debug, Default, Copy, Clone)]
struct Particle {
    pub id: usize,
    pub position: Coords,
    pub velocity: Coords,
    pub acceleration: Coords,
}

impl Particle {
    pub fn with_id(self, id: usize) -> Self {
        Self { id, ..self }
    }

    pub fn move_one_tick(&self) -> Self {
        let velocity = self.velocity + self.acceleration;
        let position = self.position + velocity;
        Self { position, velocity, ..*self }
    }
}

impl FromStr for Particle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let re = REGEX.get_or_init(|| {
            Regex::new(r"^p=(?<p>\(.+\)),\s*v=(?<v>\(.+\)),\s*a=(?<a>\(.+\))$").unwrap()
        });

        let captures = re
            .captures(s)
            .unwrap_or_else(|| panic!("invalid Particle value: {s}"));
        Ok(Self {
            position: captures.ez_get("p"),
            velocity: captures.ez_get("v"),
            acceleration: captures.ez_get("a"),
            ..Self::default()
        })
    }
}

#[derive(Debug, Clone)]
struct Universe(Vec<Particle>);

impl Universe {
    fn new<I>(particles: I) -> Self
    where
        I: Iterator<Item = Particle>,
    {
        Self(
            particles
                .sorted_by_key(|p| p.position)
                .dedup_by(|p1, p2| p1.position == p2.position)
                .collect_vec(),
        )
    }

    pub fn move_one_tick(&self) -> Self {
        Self::new(self.0.iter().map(Particle::move_one_tick))
    }

    pub fn particle_ids(&self) -> impl Iterator<Item = usize> + '_ {
        self.0.iter().map(|p| p.id)
    }
}

impl Deref for Universe {
    type Target = [Particle];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Universe {
    fn default() -> Self {
        Self::new(
            INPUT
                .lines()
                .enumerate()
                .map(|(id, line)| line.parse::<Particle>().unwrap().with_id(id)),
        )
    }
}
