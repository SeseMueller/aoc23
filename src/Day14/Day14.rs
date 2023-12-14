use std::collections::HashMap;

fn main() {
    let text = include_str!("TEMP.txt");
    // let text = include_str!("TEMP2.txt");
    // let text = include_str!("TEMP3.txt");

    // Read the 2D Grid from the text file
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in text.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    let mut grid2 = grid.clone(); // Part2

    // Swap each O with a below . until nothing is left
    loop {
        let mut changed = false;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '.' && i < grid.len() - 1 && grid[i + 1][j] == 'O' {
                    grid[i][j] = 'O';
                    grid[i + 1][j] = '.';
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }

    // Sum up the distances of each O to the bottom
    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                sum += grid.len() - i;
            }
        }
    }

    println!("Sum: {}", sum);

    // Part2
    let mut known_grids = HashMap::new();

    let mut i = 0;

    // Spin the grid 1000000000 times
    while i < 1000000000 {
        spin(&mut grid2);

        // Check if we've seen this grid before
        if known_grids.contains_key(&grid2) {
            // println!("Seen before at {}! Currently {}", known_grids[&grid2], i);

            let diff = i - known_grids[&grid2];

            // Increase i by the difference between the current iteration and the last time we saw this grid
            // until we reach 1000000000
            while i < 1000000000 {
                i += diff;
            }
            i -= diff;
        }

        // Add this grid to the set
        known_grids.insert(grid2.clone(), i);

        i += 1;
    }

    // Sum up the distances of each O to the bottom
    let mut sum = 0;
    for i in 0..grid2.len() {
        for j in 0..grid2[i].len() {
            if grid2[i][j] == 'O' {
                sum += grid2.len() - i;
            }
        }
    }

    // // Print the grid
    // for i in 0..grid2.len() {
    //     for j in 0..grid2[i].len() {
    //         print!("{}", grid2[i][j]);
    //     }
    //     println!("");
    // }

    println!("Sum: {}", sum);
}

// "Spin the grid": Do the above but first north, then west, then south, then east
fn spin(v: &mut Vec<Vec<char>>) {
    let mut changed = true;
    
    // North
    while changed {
        changed = false;

        for i in (0..v.len() - 1).rev() {
            for j in 0..v[i].len() {
                if v[i][j] == '.' && v[i + 1][j] == 'O' {
                    v[i][j] = 'O';
                    v[i + 1][j] = '.';
                    changed = true;
                }
            }
        }
    }

    changed = true;

    // West
    while changed {
        changed = false;
        
        for i in 0..v.len() {
            for j in (0..v[i].len() - 1).rev() {
                if v[i][j] == '.' && v[i][j + 1] == 'O' {
                    v[i][j] = 'O';
                    v[i][j + 1] = '.';
                    changed = true;
                }
            }
        }
    }
    
    changed = true;

    // South
    while changed {
        changed = false;
        for i in 1..v.len() {
            for j in 0..v[i].len() {
                if v[i][j] == '.' && v[i - 1][j] == 'O' {
                    v[i][j] = 'O';
                    v[i - 1][j] = '.';
                    changed = true;
                }
            }
        }
    }
    
    changed = true;
    
    // East
    while changed {
        changed = false;
        for i in 0..v.len() {
            for j in 1..v[i].len() {
                if v[i][j] == '.' && v[i][j - 1] == 'O' {
                    v[i][j] = 'O';
                    v[i][j - 1] = '.';
                    changed = true;
                }
            }
        }
    }


}
