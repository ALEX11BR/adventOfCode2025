use itertools::iproduct;

fn is_point_roll(point: (i32, i32), map: &Vec<Vec<bool>>) -> bool {
    point.0 >= 0
        && point.1 >= 0
        && point.0 < map.len() as i32
        && point.1 < map[0].len() as i32
        && map[point.0 as usize][point.1 as usize]
}

fn main() {
    let map = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().map(|c| c == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let ans = iproduct!(0..map.len(), 0..map[0].len())
        .filter(|&(i, j)| {
            map[i][j]
                && iproduct!(-1..=1, -1..=1)
                    .filter(|&(ii, jj)| {
                        (ii != 0 || jj != 0) && is_point_roll((ii + i as i32, jj + j as i32), &map)
                    })
                    .count()
                    < 4
        })
        .count();

    println!("{}", ans);
}
