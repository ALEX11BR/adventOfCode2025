use std::collections::{HashMap, HashSet, VecDeque};

use memoize::memoize;

fn bfs(map: &HashMap<String, Vec<String>>, start: &str, targets: &[&str]) -> Vec<String> {
    let mut ans = vec![];
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(start);

    while let Some(node) = q.pop_front() {
        if visited.contains(node) {
            continue;
        }
        visited.insert(node.to_owned());

        if targets.contains(&node) {
            ans.push(node.to_owned());
            if ans.len() == targets.len() {
                break;
            }
        }
        for neighbor in map.get(node).iter().flat_map(|v| *v) {
            q.push_back(neighbor);
        }
    }

    ans
}

#[memoize(Ignore: map)]
fn dfs(map: &HashMap<String, Vec<String>>, start: String, end: String) -> u64 {
    if start == end {
        1
    } else {
        map.get(&start)
            .iter()
            .flat_map(|v| *v)
            .map(|neighbor| dfs(map, neighbor.clone(), end.clone()))
            .sum()
    }
}

fn main() {
    let map = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<_>>();
            (
                parts[0].to_owned(),
                parts[1]
                    .split(' ')
                    .map(|d| d.to_owned())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    let waypoints = bfs(&map, "svr", &["svr", "fft", "dac", "out"]);

    let ans = (0..waypoints.len().saturating_sub(1))
        .map(|i| dfs(&map, waypoints[i].clone(), waypoints[i + 1].clone()))
        .product::<u64>();

    println!("{}", ans);
}
