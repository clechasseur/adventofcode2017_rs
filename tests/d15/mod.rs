use adventofcode2017_clp::day_15::{part_1, part_2};

#[test]
#[cfg_attr(not(feature = "slow"), ignore = "runs in ~2s, use `--features slow` to enable")]
fn day_15_part_1() {
    assert_eq!(638, part_1());
}

#[test]
fn day_15_part_2() {
    assert_eq!(343, part_2());
}
