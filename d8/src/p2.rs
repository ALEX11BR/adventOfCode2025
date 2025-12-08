fn main() {
    let coords = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<f64>().unwrap_or(0.))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut connections = (0..coords.len().saturating_sub(1))
        .flat_map(|i| {
            (i.saturating_add(1)..coords.len())
                .map(|j| {
                    (
                        (0..=2)
                            .map(|k| (coords[i][k] - coords[j][k]).powi(2))
                            .sum::<f64>()
                            .sqrt(),
                        (i, j),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    connections.sort_by(|c1, c2| c1.0.total_cmp(&c2.0));

    let mut visited = vec![false; coords.len()];
    let mut visited_count = 0;

    for (_, (i, j)) in connections {
        if !visited[i] {
            visited[i] = true;
            visited_count += 1;
        }
        if !visited[j] {
            visited[j] = true;
            visited_count += 1;
        }

        if visited_count == coords.len() {
            let ans = coords[i][0] * coords[j][0];
            println!("{}", ans);
            return;
        }
    }
}
