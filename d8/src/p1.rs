use std::collections::HashSet;

const SHORTEST_CONNECTIONS: usize = 1000;

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
    connections.truncate(SHORTEST_CONNECTIONS);

    let graph = (0..coords.len())
        .map(|i| {
            connections
                .iter()
                .filter_map(|(_, (ii, jj))| {
                    if *ii == i {
                        Some(*jj)
                    } else if *jj == i {
                        Some(*ii)
                    } else {
                        None
                    }
                })
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();

    let mut visited = vec![false; coords.len()];
    let mut circuits = vec![];

    for i in 0..coords.len() {
        if !visited[i] {
            visited[i] = true;

            let mut q = vec![i];
            let mut circuit_size = 0;

            while let Some(elem) = q.pop() {
                circuit_size += 1;

                for neigh in graph[elem].iter() {
                    if !visited[*neigh] {
                        visited[*neigh] = true;
                        q.push(*neigh);
                    }
                }
            }

            circuits.push(circuit_size);
        }
    }

    circuits.sort_by(|c1, c2| c1.cmp(c2).reverse());
    let ans = circuits[0] * circuits[1] * circuits[2];

    println!("{}", ans);
}
