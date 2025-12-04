use itertools::iproduct;

fn is_point_roll(point: (i32, i32), map: &Vec<Vec<bool>>) -> bool {
    point.0 >= 0
        && point.1 >= 0
        && point.0 < map.len() as i32
        && point.1 < map[0].len() as i32
        && map[point.0 as usize][point.1 as usize]
}

fn main() {
    let mut map = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().map(|c| c == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let ans = std::iter::repeat(())
        .map_while(|_| {
            let elems_to_remove = iproduct!(0..map.len(), 0..map[0].len())
                .filter(|&(i, j)| {
                    map[i][j]
                        && iproduct!(-1..=1, -1..=1)
                            .filter(|&(ii, jj)| {
                                (ii != 0 || jj != 0)
                                    && is_point_roll((ii + i as i32, jj + j as i32), &map)
                            })
                            .count()
                            < 4
                })
                .collect::<Vec<_>>();

            elems_to_remove.iter().for_each(|&(i, j)| {
                map[i][j] = false;
            });

            if elems_to_remove.is_empty() {
                None
            } else {
                Some(elems_to_remove.len())
            }
        })
        .sum::<usize>();

    println!("{}", ans);
}
