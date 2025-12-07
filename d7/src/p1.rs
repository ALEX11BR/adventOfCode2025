use std::collections::BTreeSet;

fn main() {
    let world = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = (0..world[0].len())
        .find(|&j| world[0][j] == 'S')
        .unwrap_or(0);

    let mut set = BTreeSet::new();
    set.insert(start);

    let (ans, _) = (1..world.len()).fold((0, set), |(ans, columns), i| {
        let (new_columns, splits) = columns.iter().fold(
            (BTreeSet::new(), 0),
            |(mut new_columns, splits), &column| {
                if world[i][column] == '^' {
                    new_columns.insert(column.saturating_sub(1));
                    new_columns.insert(column.saturating_add(1));
                    (new_columns, splits + 1)
                } else {
                    new_columns.insert(column);
                    (new_columns, splits)
                }
            },
        );

        (ans + splits, new_columns)
    });

    println!("{}", ans);
}
