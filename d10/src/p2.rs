use std::error::Error;

use microlp::{ComparisonOp, OptimizationDirection, Problem};

fn main() -> Result<(), Box<dyn Error>> {
    let line_regex = regex::Regex::new(r"^\[([.#]*)\] (.*) \{([0-9,]*)\}$")?;

    let ans = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let Some(cap) = line_regex.captures(&line) else {
                return 0;
            };

            let buttons = cap[2]
                .split(' ')
                .map(|btn| {
                    btn[1..btn.len().saturating_sub(1)]
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap_or(0))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let mut problem = Problem::new(OptimizationDirection::Minimize);
            let button_vars = (0..buttons.len())
                .map(|_| problem.add_integer_var(1., (0, std::i32::MAX)))
                .collect::<Vec<_>>();

            cap[3]
                .split(',')
                .map(|num| num.parse::<u64>().unwrap_or(0))
                .enumerate()
                .for_each(|(i, amount)| {
                    problem.add_constraint(
                        (0..buttons.len())
                            .filter(|&j| buttons[j].contains(&i))
                            .map(|j| (button_vars[j], 1.0)),
                        ComparisonOp::Eq,
                        amount as f64,
                    );
                });

            let Ok(solution) = problem.solve() else {
                return std::u64::MAX;
            };

            solution.objective().round() as u64
        })
        .sum::<u64>();

    println!("{}", ans);

    Ok(())
}
