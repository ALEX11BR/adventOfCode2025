use std::collections::HashMap;

fn dfs(map: &HashMap<String, Vec<String>>, start: &str, end: &str) -> u64 {
    let mut q = vec![start];
    let mut ans = 0;

    while let Some(node) = q.pop() {
        if node == end {
            ans += 1;
            continue;
        }

        map.get(node)
            .into_iter()
            .flatten()
            .for_each(|neigh| q.push(neigh));
    }

    ans
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

    let ans = dfs(&map, "you", "out");

    println!("{}", ans);
}
