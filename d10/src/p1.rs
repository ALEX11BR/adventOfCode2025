use std::{
    collections::{HashSet, VecDeque},
    error::Error,
};

fn solve(final_state: u32, buttons: &Vec<u32>) -> u64 {
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    q.push_front((0, 0));

    while let Some((state, dist)) = q.pop_back() {
        if visited.contains(&state) {
            continue;
        }
        if state == final_state {
            return dist;
        }
        visited.insert(state);

        for button in buttons {
            q.push_front((state ^ button, dist + 1));
        }
    }

    std::u64::MAX
}

fn main() -> Result<(), Box<dyn Error>> {
    let line_regex = regex::Regex::new(r"^\[([.#]*)\] (.*) \{([0-9,]*)\}$")?;

    let ans = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let Some(cap) = line_regex.captures(&line) else {
                return std::u64::MAX;
            };
            let final_state = cap[1]
                .chars()
                .map(|c| c == '#')
                .enumerate()
                .map(|(i, active)| (active as u32) << i)
                .sum::<u32>();
            let buttons = cap[2]
                .split(' ')
                .map(|btn| {
                    btn[1..btn.len().saturating_sub(1)]
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap_or(0))
                        .map(|n| 1 << n)
                        .sum::<u32>()
                })
                .collect::<Vec<_>>();

            solve(final_state, &buttons)
        })
        .sum::<u64>();

    println!("{}", ans);

    Ok(())
}
