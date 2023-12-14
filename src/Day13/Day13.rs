use itertools::Itertools;

fn main() {
    let text = include_str!("TEMP.txt");
    // let text = include_str!("TEMP2.txt");
    // let text = include_str!("TEMP3.txt");

    // Read in Grids of . and #, seperated by \n\n

    let grids = text.split("\n\n").map(|s| s.trim()).collect::<Vec<&str>>();

    let mut sum = 0;
    let mut sum2 = 0;

    for grid in grids.clone() {
        // Find the reflection collumn or row

        let transposed = grid
            .split("\n")
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let transposed = (0..transposed[0].len())
            .map(|i| transposed.iter().map(|inner| inner[i]).collect::<String>())
            .collect::<Vec<String>>();

        // 2 or 3 Rows that are the same
        let possible_rows = grid
            .split("\n")
            .collect::<Vec<&str>>()
            .windows(3)
            .enumerate()
            .filter_map(|x| {
                if x.1[0] == x.1[1] {
                    Some(x.0)
                } else if x.1[1] == x.1[2] {
                    Some(x.0 + 1)
                // } else if x.1[0] == x.1[2] {
                //     Some((x.0, true))
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();

        let possible_collumns = transposed
            .windows(3)
            .enumerate()
            .filter_map(|x| {
                if x.1[0] == x.1[1] {
                    Some(x.0)
                } else if x.1[1] == x.1[2] {
                    Some(x.0 + 1)
                // } else if x.1[0] == x.1[2] {
                //     Some((x.0, true))
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();

        let mut really_possible_rows = Vec::new();

        let grid_rows = grid.split("\n").collect::<Vec<&str>>();
        for row in possible_rows {
            let mut possible = true;

            let mut left_start = row;
            let mut right_start = row + 1;

            while left_start > 0 && right_start < grid_rows.len() - 1 {
                if grid_rows[left_start - 1] != grid_rows[right_start + 1] {
                    possible = false;
                    // dbg!(grid_rows[left_start - 1], grid_rows[right_start + 1]);
                    break;
                }

                left_start -= 1;
                right_start += 1;
            }

            if possible {
                really_possible_rows.push(row);
            }
        }

        let mut really_possible_collumns = Vec::new();

        for column in possible_collumns {
            let mut possible = true;

            let mut left_start = column;
            let mut right_start = column + 1;

            let grid_columns = transposed.clone();

            while left_start > 0 && right_start < grid_columns.len() - 1 {
                if grid_columns[left_start - 1] != grid_columns[right_start + 1] {
                    possible = false;
                    // dbg!(grid_columns[left_start - 1], grid_columns[right_start + 1]);
                    break;
                }

                left_start -= 1;
                right_start += 1;
            }

            if possible {
                really_possible_collumns.push(column);
            }
        }

        // Get unique rows and collumns
        let really_possible_rows = really_possible_rows
            .iter()
            .unique()
            .collect::<Vec<&usize>>();
        let really_possible_collumns = really_possible_collumns
            .iter()
            .unique()
            .collect::<Vec<&usize>>();

        if really_possible_rows.len() + really_possible_collumns.len() != 1 {
            dbg!(
                really_possible_rows.clone(),
                really_possible_collumns.clone()
            );
            dbg!(grid_rows.clone(), transposed.clone());
        }

        for row in really_possible_rows {
            sum += 100 * (row + 1);
        }

        for column in really_possible_collumns {
            sum += column + 1;
        }

        // Part 2
        // A mirror is a mirror iff its reflection differs in exactly one location.

        let possible_rows = (0..grid_rows.len()).collect::<Vec<usize>>();
        let mut really_possible_rows = Vec::new();

        for row in possible_rows {
            let mut differences = 0;

            let mut left_start = row;
            let mut right_start = row + 1;

            while right_start < grid_rows.len() {
                let diff = grid_rows[left_start]
                    .chars()
                    .zip(grid_rows[right_start].chars())
                    .filter(|x| x.0 != x.1)
                    .count();

                differences += diff;

                if differences > 1 {
                    break;
                };

                match left_start.checked_sub(1){
                    Some(x) => left_start = x,
                    None => break,
                }
                right_start += 1;

            }

            if differences == 1 {
                really_possible_rows.push(row);
            }
        }

        let possible_collumns = (0..transposed.len()).collect::<Vec<usize>>();
        let mut really_possible_collumns = Vec::new();

        for column in possible_collumns {
            let mut differences = 0;

            let mut left_start = column;
            let mut right_start = column + 1;

            while right_start < transposed.len() {
                let diff = transposed[left_start]
                    .chars()
                    .zip(transposed[right_start].chars())
                    .filter(|x| x.0 != x.1)
                    .count();

                differences += diff;

                if differences > 1 {
                    break;
                };

                match left_start.checked_sub(1){
                    Some(x) => left_start = x,
                    None => break,
                }
                right_start += 1;

            }

            if differences == 1 {
                really_possible_collumns.push(column);
            }
        }

        // Get unique rows and collumns
        let really_possible_rows = really_possible_rows
            .iter()
            .unique()
            .collect::<Vec<&usize>>();

        let really_possible_collumns = really_possible_collumns
            .iter()
            .unique()
            .collect::<Vec<&usize>>();

        if really_possible_rows.len() + really_possible_collumns.len() != 1 {
            dbg!(
                really_possible_rows.clone(),
                really_possible_collumns.clone()
            );
            dbg!(grid_rows.clone(), transposed.clone());
        }

        for row in really_possible_rows {
            sum2 += 100 * (row + 1);
        }

        for column in really_possible_collumns {
            sum2 += column + 1;
        }

        // dbg!(really_possible_rows, really_possible_collumns);

    }

    println!("{}", sum);
    println!("{}", sum2);
}
