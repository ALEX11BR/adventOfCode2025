use std::cmp::max;

const NUM_DIGITS: usize = 12;

fn main() {
    let ans = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.chars()
                .map(|c| Into::<u64>::into(c) - Into::<u64>::into('0'))
                .collect::<Vec<_>>()
        })
        .map(|line| {
            let line_len = line.len();
            let mut dyn_arr = vec![0; line_len];

            for i in 0..NUM_DIGITS {
                let mut new_dyn = vec![0; line_len];
                new_dyn[i] = if i > 0 { dyn_arr[i - 1] * 10 } else { 0 } + line[i];
                for j in (i + 1)..line_len {
                    new_dyn[j] = max(new_dyn[j - 1], dyn_arr[j - 1] * 10 + line[j]);
                }

                dyn_arr = new_dyn;
            }

            dyn_arr[line_len - 1]
        })
        .sum::<u64>();

    println!("{}", ans);
}
