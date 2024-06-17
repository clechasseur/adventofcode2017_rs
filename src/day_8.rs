use std::cmp::max;
use std::collections::HashMap;

use regex_static::static_regex;

use crate::input::day_8::INPUT;

pub fn part_1() -> i64 {
    final_registers().values().max().unwrap()
}

pub fn part_2() -> i64 {
    final_registers().max_ever
}

#[derive(Debug, Default)]
struct Registers<'a> {
    registers: HashMap<&'a str, i64>,
    max_ever: i64,
}

impl<'a> Registers<'a> {
    fn get(&self, name: &'a str) -> i64 {
        self.registers.get(name).copied().unwrap_or_default()
    }

    fn update<F>(&mut self, name: &'a str, f: F)
    where
        F: FnOnce(i64) -> i64,
    {
        let register = self.registers.entry(name).or_default();
        *register = f(*register);
        self.max_ever = max(self.max_ever, *register);
    }

    fn values(&self) -> impl Iterator<Item = i64> + '_ {
        self.registers.values().copied()
    }
}

#[derive(Debug)]
struct Instruction<'a> {
    register: &'a str,
    offset: i64,
    cmp_register: &'a str,
    cmp_op: &'a str,
    cmp_value: i64,
}

impl<'a> Instruction<'a> {
    fn apply(&self, registers: &mut Registers<'a>) {
        let cmp_register = registers.get(self.cmp_register);
        if self.apply_op(cmp_register, self.cmp_value) {
            registers.update(self.register, |r| r + self.offset);
        }
    }

    fn apply_op(&self, a: i64, b: i64) -> bool {
        match self.cmp_op {
            "<" => a < b,
            "<=" => a <= b,
            ">" => a > b,
            ">=" => a >= b,
            "==" => a == b,
            "!=" => a != b,
            op => panic!("expected one of '<', '<=', '>', '>=', '==' or '!=', got '{}'", op),
        }
    }
}

impl<'a> From<&'a str> for Instruction<'a> {
    fn from(value: &'a str) -> Self {
        let parser =
            static_regex!(r"^([a-z]+) (inc|dec) (-?\d+) if ([a-z]+) (<=?|>=?|[=!]=) (-?\d+)$");

        let (_, [register, op, offset, cmp_register, cmp_op, cmp_value]) =
            parser.captures(value).unwrap().extract();
        Self {
            register,
            offset: offset
                .parse::<i64>()
                .map(|o| if op == "dec" { -o } else { o })
                .unwrap(),
            cmp_register,
            cmp_op,
            cmp_value: cmp_value.parse().unwrap(),
        }
    }
}

fn final_registers() -> Registers<'static> {
    let mut registers = Registers::default();
    INPUT
        .iter()
        .cloned()
        .map(Into::<Instruction>::into)
        .for_each(|instruction| instruction.apply(&mut registers));
    registers
}
