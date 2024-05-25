use adventofcode2017_clp::day_5::{part_1, part_2};

#[test]
fn day_5_part_1() {
    assert_eq!(339_351, part_1());
}

#[test]
#[cfg_attr(not(feature = "slow"), ignore = "runs in ~2s, use `--features slow` to enable")]
fn day_5_part_2() {
    assert_eq!(24_315_397, part_2());
}
