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

                (2..=digits.len()).any(|len_rep| {
                    if digits.len() % len_rep != 0 {
                        return false;
                    }
                    for i in 0..(digits.len() / len_rep) {
                        for j in 1..len_rep {
                            if digits[i] != digits[i + j * digits.len() / len_rep] {
                                return false;
                            }
                        }
                    }
                    true
                })
            })
        })
        .sum::<u64>();

    println!("{}", ans);

    Ok(())
}
