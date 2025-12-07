fn main() {
    let world = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = (0..world[0].len())
        .find(|&j| world[0][j] == 'S')
        .unwrap_or(0);

    let mut start_array = vec![0u64; world[0].len()];
    start_array[start] = 1;

    let ans = (1..world.len())
        .fold(start_array, |ways, i| {
            let mut new_ways = vec![0; world[0].len()];

            for j in 0..world[0].len() {
                if world[i][j] == '^' {
                    new_ways[j.saturating_sub(1)] += ways[j];
                    new_ways[j.saturating_add(1)] += ways[j];
                } else {
                    new_ways[j] += ways[j];
                }
            }

            new_ways
        })
        .iter()
        .sum::<u64>();

    println!("{}", ans);
}
