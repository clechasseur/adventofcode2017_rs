use adventofcode2017_clp::day_25::part_1;

#[test]
#[cfg_attr(not(feature = "slow"), ignore = "runs in ~4s, use `--features slow` to enable")]
fn day_25_part_1() {
    assert_eq!(2_526, part_1());
}
