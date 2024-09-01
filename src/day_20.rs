use std::iter::successors;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::OnceLock;

use itertools::Itertools;
use regex::Regex;

use crate::helpers::pt_3d::Pt3d;
use crate::helpers::regex::EzCapturesHelper;
use crate::input::day_20::INPUT;

pub fn part_1() -> usize {
    Universe::default()
        .iter()
        .sorted_unstable_by(|p1, p2| {
            p1.abs_acceleration()
                .cmp(&p2.abs_acceleration())
                .then_with(|| p1.abs_velocity().cmp(&p2.abs_velocity()))
                .then_with(|| p1.abs_position().cmp(&p2.abs_position()))
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

    pub fn move_one_tick(self) -> Self {
        let velocity = self.velocity + self.acceleration;
        let position = self.position + velocity;
        Self { position, velocity, ..self }
    }

    pub fn abs_position(&self) -> i64 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    pub fn abs_velocity(&self) -> i64 {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }

    pub fn abs_acceleration(&self) -> i64 {
        self.acceleration.x.abs() + self.acceleration.y.abs() + self.acceleration.z.abs()
    }
}

impl FromStr for Particle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let re = REGEX.get_or_init(|| {
            Regex::new(r"^p=(?<p>\(.+\)),\s*v=(?<v>\(.+\)),\s*a=(?<a>\(.+\))$").unwrap()
        });

        let captures = re.ez_captures(s, "Particle");
        Ok(Self {
            position: captures.get("p"),
            velocity: captures.get("v"),
            acceleration: captures.get("a"),
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
        Self::new(self.0.iter().copied().map(Particle::move_one_tick))
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
