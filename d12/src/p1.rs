fn main() {
    let mut shapes = vec![];
    let mut configs = vec![];

    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            if line.contains('x') {
                let line_parts = line.split(": ").collect::<Vec<_>>();
                let sizes = line_parts[0]
                    .split('x')
                    .map(|num| num.parse::<usize>().unwrap_or(0))
                    .collect::<Vec<_>>();
                let shape_targets = line_parts[1]
                    .split(' ')
                    .map(|num| num.parse().unwrap_or(0))
                    .collect::<Vec<_>>();

                configs.push((sizes[0], sizes[1], shape_targets));
            } else if line.contains(':') {
                shapes.push(vec![]);
            } else if !line.is_empty() {
                if let Some(shape_to_add) = shapes.last_mut() {
                    shape_to_add.push(line.chars().map(|c| c == '#').collect::<Vec<_>>());
                }
            }
        });

    let tiles_per_shape = shapes
        .iter()
        .map(|shape| shape.iter().flatten().filter(|e| **e).count())
        .collect::<Vec<_>>();

    let ans = configs
        .iter()
        .filter(|(width, height, demands)| {
            width * height
                >= demands
                    .iter()
                    .enumerate()
                    .map(|(i, demand)| tiles_per_shape[i] * demand)
                    .sum()
        })
        .count();

    println!("{}", ans);
}
