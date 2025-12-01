const WHEEL_SIZE: i32 = 100;

fn num_of_zero_pos(pos: i32, move_by: i32) -> i32 {
    let raw_pos = pos + move_by;

    if raw_pos > 0 {
        raw_pos / WHEEL_SIZE
    } else {
        (raw_pos - 1).div_euclid(WHEEL_SIZE).abs() - (pos == 0) as i32
    }
}

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

            (ans + num_of_zero_pos(pos, move_by), new_pos)
        });

    println!("{}", ans);
}
