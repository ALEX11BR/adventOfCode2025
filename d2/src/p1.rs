use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let ans = std::io::stdin()
        .lines()
        .next()
        .and_then(|line| line.ok())
        .unwrap_or_default()
        .split(',')
        .map(|range_str| {
            let range_bounds = range_str
                .split('-')
                .filter_map(|s| s.parse::<u64>().ok())
                .collect::<Vec<_>>();

            range_bounds[0]..=range_bounds[1]
        })
        .flat_map(|range| {
            range.filter(|elem| {
                let mut el = *elem;
                let mut digits = vec![];
                while el > 0 {
                    digits.push(el % 10);
                    el = el / 10;
                }

                if digits.len() % 2 == 1 {
                    return false;
                }
                for i in 0..(digits.len() / 2) {
                    if digits[i] != digits[i + digits.len() / 2] {
                        return false;
                    }
                }
                true
            })
        })
        .sum::<u64>();

    println!("{}", ans);

    Ok(())
}
