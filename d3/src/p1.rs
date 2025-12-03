fn main() {
    let ans = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.chars()
                .map(|c| Into::<u32>::into(c) - Into::<u32>::into('0'))
                .collect::<Vec<_>>()
        })
        .map(|line| {
            let (first_pos, first_digit) = line[..(line.len() - 1)]
                .iter()
                .enumerate()
                .max_by(|(e0_pos, e0_num), (e1_pos, e1_num)| {
                    e0_num.cmp(e1_num).then(e0_pos.cmp(e1_pos).reverse())
                })
                .unwrap_or((0, &0));

            let second_digit = line[(first_pos + 1)..].iter().max().unwrap_or(&0);

            first_digit * 10 + second_digit
        })
        .sum::<u32>();

    println!("{}", ans);
}
