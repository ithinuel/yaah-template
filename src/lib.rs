use yaah::{aoc, aoc_lib, aoc_year};

aoc_year!({{ year }});

#[aoc(day1, part1)]
fn day1_part1(_input: &'static str) -> usize {
    todo!()
}

aoc_lib!({% if with_benchmarks%}with_benchmarks{% endif %});
