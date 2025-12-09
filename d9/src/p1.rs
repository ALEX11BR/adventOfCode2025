fn main() {
    let red_squares = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<u64>().unwrap_or(0))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let ans = (0..red_squares.len().saturating_sub(1))
        .flat_map(|i| {
            (i.saturating_add(1)..red_squares.len())
                .map(|j| {
                    red_squares[i][0]
                        .abs_diff(red_squares[j][0])
                        .saturating_add(1)
                        * red_squares[i][1]
                            .abs_diff(red_squares[j][1])
                            .saturating_add(1)
                })
                .collect::<Vec<_>>()
        })
        .max()
        .unwrap_or(0);

    println!("{}", ans);
}
