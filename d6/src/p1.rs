fn main() {
    let elems = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let ans = (0..elems[0].len())
        .map(|j| {
            let digits_in_column = (0..elems.len().saturating_sub(1))
                .map(|i| elems[i][j].parse::<usize>().unwrap_or_default());

            match elems[elems.len() - 1][j].as_str() {
                "+" => digits_in_column.sum(),
                "*" => digits_in_column.product(),
                _ => 0,
            }
        })
        .sum::<usize>();

    println!("{}", ans);
}
