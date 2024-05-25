use std::iter;

use crate::input::day_5::INPUT;

pub fn part_1() -> usize {
    steps(false)
}

pub fn part_2() -> usize {
    steps(true)
}

fn steps(strange: bool) -> usize {
    // Skip the initial state, but count the last jump.
    maze(strange).skip(1).count() + 1
}

fn maze(strange: bool) -> impl Iterator<Item = usize> {
    let mut jumps: Vec<_> = INPUT.into();
    let mut offset = 0usize;

    iter::from_fn(move || {
        jumps.get_mut(offset).map(|jmp| {
            let cur_offset = offset;
            offset = offset.wrapping_add_signed(*jmp);
            *jmp += if strange && *jmp >= 3 { -1 } else { 1 };
            cur_offset
        })
    })
}
