const WHEEL_SIZE: i32 = 100;

fn main() {
    let (ans, _) = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            (if line.chars().next() == Some('L') {
                -1
            } else {
                1
            }) * &line[1..].parse::<i32>().unwrap_or(0)
        })
        .fold((0, 50), |(ans, pos), move_by| {
            let new_pos = (pos + move_by).rem_euclid(WHEEL_SIZE);

            (ans + (new_pos == 0) as i32, new_pos)
        });

    println!("{}", ans);
}
