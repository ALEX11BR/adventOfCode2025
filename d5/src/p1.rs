use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let range_regex = regex::Regex::new("^([0-9]*)-([0-9]*)$")?;

    let ranges = std::io::stdin()
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
        .collect::<Vec<_>>();

    let ans = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<usize>().ok())
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count();

    println!("{}", ans);

    Ok(())
}
