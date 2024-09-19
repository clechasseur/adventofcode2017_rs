use adventofcode2017_clp::day_21::{part_1, part_2};

#[test]
fn day_21_part_1() {
    assert_eq!(188, part_1());
}

#[test]
#[cfg_attr(not(feature = "slow"), ignore = "runs in ~2s, use `--features slow` to enable")]
fn day_21_part_2() {
    assert_eq!(2_758_764, part_2());
}
