use adventofcode2017_clp::day_22::{part_1, part_2};

#[test]
fn day_22_part_1() {
    assert_eq!(5_411, part_1());
}

#[test]
#[cfg_attr(not(feature = "slow"), ignore = "runs in ~6s, use `--features slow` to enable")]
fn day_22_part_2() {
    assert_eq!(2_511_416, part_2());
}
