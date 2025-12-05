use std::{
    cmp::{max, min},
    error::Error,
    ops::RangeInclusive,
};

fn intersect_ranges(
    old_range: &RangeInclusive<usize>,
    new_range: &RangeInclusive<usize>,
) -> Vec<RangeInclusive<usize>> {
    let mut ans = vec![];

    if new_range.start() < old_range.start() {
        ans.push(*new_range.start()..=min(old_range.start() - 1, *new_range.end()));
    }
    if old_range.end() < new_range.end() {
        ans.push(max(*old_range.end() + 1, *new_range.start())..=*new_range.end());
    }

    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    let range_regex = regex::Regex::new("^([0-9]*)-([0-9]*)$")?;

    let ans = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map_while(|line| {
            range_regex.captures(&line).map(|caps| {
                let start = caps
                    .get(1)
                    .and_then(|s| s.as_str().parse::<usize>().ok())
                    .unwrap_or_default();
                let end = caps
                    .get(2)
                    .and_then(|s| s.as_str().parse::<usize>().ok())
                    .unwrap_or_default();

                start..=end
            })
        })
        .fold(vec![], |old_ranges, new_range| {
            let mut old_ranges = old_ranges;

            old_ranges.append(&mut old_ranges.iter().fold(
                vec![new_range],
                |new_ranges, old_range| {
                    new_ranges
                        .iter()
                        .flat_map(|new_range| intersect_ranges(old_range, new_range))
                        .collect()
                },
            ));

            old_ranges
        })
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<usize>();

    println!("{}", ans);

    Ok(())
}
