use std::convert::identity;


fn main() {

    let text = include_str!("TEMP.txt");
    // let text = include_str!("TEMP2.txt");
    // let text = include_str!("TEMP3.txt");

    // There are stars and there is nothing between the stars.
    // The comulative distance between al star pairs is to be calculated.
    // BUT; all rows and colloums that are empty are to be doubled.

    let grid = text.lines().map(|line| {
        line.chars().map(|c| {
            match c {
                '.' => false,
                '#' => true,
                _ => panic!("Unknown character: {}", c),
            }
        }).collect::<Vec<bool>>()
    }).collect::<Vec<Vec<bool>>>();

    let empty_rows:Vec<usize> = grid.iter().enumerate().filter_map(|(y, row)| {
        if row.iter().all(|&b| !b) {
            Some(y)
        } else {
            None
        }
    }).collect();

    let empty_cols:Vec<usize> = (0..grid[0].len()).filter_map(|x| {
        if grid.iter().all(|row| !row[x]) {
            Some(x)
        } else {
            None
        }
    }).collect();

    let star_positions:Vec<(usize, usize)> = grid.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter_map(move |(x, &b)| {
            if b {
                Some((x, y))
            } else {
                None
            }
        })
    }).collect();

    // Each starts gets ++ for each empty row and col that is in that vec which has a lower index than the star.

    let new_star_positions1:Vec<(usize, usize)> = star_positions.iter().map(|&(x, y)| {
        let mut new_x = x;
        let mut new_y = y;

        for &empty_row in &empty_rows {
            if empty_row < y {
                new_y += 1;
            }
        }

        for &empty_col in &empty_cols {
            if empty_col < x {
                new_x += 1;
            }
        }

        (new_x, new_y)
    }).collect();

    let new_star_positions:Vec<(usize, usize)> = star_positions.iter().map(|&(x, y)| {
        let mut new_x = x;
        let mut new_y = y;

        for &empty_row in &empty_rows {
            if empty_row < y {
                new_y += 999999;
            }
        }

        for &empty_col in &empty_cols {
            if empty_col < x {
                new_x += 999999;
            }
        }

        (new_x, new_y)
    }).collect();

    let mut total_distance = 0;
    let mut total_distance2 = 0;
    // taxicab distance

    for (i, &(x1, y1)) in new_star_positions1.iter().enumerate() {
        for &(x2, y2) in &new_star_positions1[i+1..] {
            total_distance += (x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs();
        }
    }

    for (i, &(x1, y1)) in new_star_positions.iter().enumerate() {
        for &(x2, y2) in &new_star_positions[i+1..] {
            total_distance2 += (x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs();
        }
    }

    println!("Total distance: {}", total_distance);
    println!("Total distance2: {}", total_distance2);

}