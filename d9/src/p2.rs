use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SegmentStatus {
    Start,
    Middle,
    End,
}

impl SegmentStatus {
    fn is_collision(&self, start: bool) -> bool {
        match *self {
            SegmentStatus::Middle => true,
            SegmentStatus::Start => start,
            SegmentStatus::End => !start,
        }
    }
}

fn get_segment_status(start: u64, test: u64, end: u64) -> Option<SegmentStatus> {
    if test == start {
        Some(SegmentStatus::Start)
    } else if test == end {
        Some(SegmentStatus::End)
    } else if start < test && test < end {
        Some(SegmentStatus::Middle)
    } else {
        None
    }
}

fn junctions_check(
    junctions: &Vec<(u64, SegmentStatus)>,
    start_other: u64,
    end_other: u64,
    start: bool,
) -> bool {
    let id_start = junctions.partition_point(|e| e.0 <= start_other);
    let id_end = junctions.partition_point(|e| e.0 < end_other);

    (id_start..id_end).all(|id| !junctions[id].1.is_collision(start))
}

fn main() {
    let red_squares = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<u64>().unwrap_or(0))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let edges_iter =
        (0..red_squares.len()).map(|i| (i, i.saturating_add(1).rem_euclid(red_squares.len())));

    let column_junctions = red_squares
        .iter()
        .map(|point| point[0])
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|column| {
            let mut junctions = edges_iter
                .clone()
                .filter_map(|(i, j)| {
                    let start = min(red_squares[i][0], red_squares[j][0]);
                    let end = max(red_squares[i][0], red_squares[j][0]);

                    (red_squares[i][1] == red_squares[j][1])
                        .then(|| get_segment_status(start, column, end))
                        .flatten()
                        .map(|s| (red_squares[i][1], s))
                })
                .collect::<Vec<_>>();
            junctions.sort();

            (column, junctions)
        })
        .collect::<HashMap<_, _>>();

    let line_junctions = red_squares
        .iter()
        .map(|point| point[1])
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|line| {
            let mut junctions = edges_iter
                .clone()
                .filter_map(|(i, j)| {
                    let start = min(red_squares[i][1], red_squares[j][1]);
                    let end = max(red_squares[i][1], red_squares[j][1]);

                    (red_squares[i][0] == red_squares[j][0])
                        .then(|| get_segment_status(start, line, end))
                        .flatten()
                        .map(|s| (red_squares[i][0], s))
                })
                .collect::<Vec<_>>();
            junctions.sort();

            (line, junctions)
        })
        .collect::<HashMap<_, _>>();

    let ans = (0..red_squares.len().saturating_sub(1))
        .flat_map(|i| {
            (i.saturating_add(1)..red_squares.len())
                .filter(|&j| {
                    let start_column = min(red_squares[i][0], red_squares[j][0]);
                    let end_column = max(red_squares[i][0], red_squares[j][0]);
                    let start_line = min(red_squares[i][1], red_squares[j][1]);
                    let end_line = max(red_squares[i][1], red_squares[j][1]);

                    junctions_check(&column_junctions[&start_column], start_line, end_line, true)
                        && junctions_check(
                            &column_junctions[&end_column],
                            start_line,
                            end_line,
                            false,
                        )
                        && junctions_check(
                            &line_junctions[&start_line],
                            start_column,
                            end_column,
                            true,
                        )
                        && junctions_check(
                            &line_junctions[&end_line],
                            start_column,
                            end_column,
                            false,
                        )
                })
                .map(|j| {
                    red_squares[i][0]
                        .abs_diff(red_squares[j][0])
                        .saturating_add(1)
                        * red_squares[i][1]
                            .abs_diff(red_squares[j][1])
                            .saturating_add(1)
                })
                .collect::<Vec<_>>()
        })
        .max()
        .unwrap_or(0);

    println!("{}", ans);
}
