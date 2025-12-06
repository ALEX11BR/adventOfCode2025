use std::{error::Error, iter::repeat};

use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

fn main() -> Result<(), Box<dyn Error>> {
    let num_regex = regex::Regex::new(r" *\d+( |$)")?;

    let lines = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>();

    let (_, mut offsets) = repeat(())
        .fold_while((0, vec![]), |(offset, mut offsets), _| {
            match lines[0..lines.len().saturating_sub(1)]
                .iter()
                .filter_map(|line| num_regex.find_at(line, offset).map(|m| m.len()))
                .max()
                .map(|w| {
                    offsets.push((offset, offset + w));
                    w
                }) {
                Some(w) => Continue((w + offset, offsets)),
                None => Done((offset, offsets)),
            }
        })
        .into_inner();
    offsets.last_mut().map(|el| {
        el.1 += 1;
    });
    let offsets = offsets;

    let ans = offsets
        .iter()
        .map(|(start, next_start)| {
            let nums = (*start..=next_start.saturating_sub(2)).filter_map(|j| {
                lines[0..lines.len().saturating_sub(1)]
                    .iter()
                    .filter_map(|line| line[j..j + 1].parse::<usize>().ok())
                    .reduce(|ans, digit| ans * 10 + digit)
            });

            match lines.last().and_then(|line| line.chars().nth(*start)) {
                Some('+') => nums.sum(),
                Some('*') => nums.product(),
                _ => 0,
            }
        })
        .sum::<usize>();

    println!("{}", ans);

    Ok(())
}
