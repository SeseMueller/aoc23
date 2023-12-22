use std::collections::VecDeque;

fn main() {
    let text = include_str!("TEMP.txt");
    // let text = include_str!("TEMP2.txt");
    // let text = include_str!("TEMP3.txt");

    // simlpe floodfill problem

    // Read grid
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in text.lines() {
        grid.push(line.chars().collect());
    }

    // Find starting point "S"
    let mut start: (usize, usize) = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 'S' {
                start = (i, j);
            }
        }
    }
    dbg!(start);

    let mut filled = 0;

    grid[start.0][start.1] = '.'; // Replace S with . to make it easier to floodfill
    let part2grid = grid.clone(); // Save grid for part 2
    {
        // Floodfill 64 times
        let mut count = 0;

        let mut queue: Vec<(usize, usize, usize)> = Vec::new(); // (i, j, depth)
        queue.push((start.0, start.1, 0));

        let mut newqueue: Vec<(usize, usize, usize)> = Vec::new();

        for u in 0..65 {
            while !queue.is_empty() {
                let (i, j, _) = queue.pop().unwrap();
                if grid[i][j] == '.' {
                    // Partity: O or H
                    grid[i][j] = if u % 2 == 0 { 'O' } else { 'H' };
                    count += 1;
                    if i > 0 {
                        newqueue.push((i - 1, j, u));
                    }
                    if i < grid.len() - 1 {
                        newqueue.push((i + 1, j, u));
                    }
                    if j > 0 {
                        newqueue.push((i, j - 1, u));
                    }
                    if j < grid[i].len() - 1 {
                        newqueue.push((i, j + 1, u));
                    }
                }
            }

            queue = newqueue.clone();
        }

        // Count filled cells with an odd parity
        for row in grid.iter() {
            for col in row.iter() {
                if *col == 'O' {
                    filled += 1;
                }
            }
        }

        println!("Part 1: {}, with parity: {}", count, filled);
    }
    // Debug print grif
    // for row in grid.iter() {
    //     for col in row.iter() {
    //         print!("{}", col);
    //     }
    //     println!();
    // }

    // Part 2: The grid loops infinitely, the step size is much higher.
    // Idea: start floodfilling from the start grid and fill it completely. Record the steps at which the squares outside the grid are filled.
    // When the grid is filled, start floodfilling the next grid with the recorded steps.
    // This way the floodfill function can be cached.

    // let max_steps = 5000; // For now; increase later to goal.
    let max_steps = 26501365;

    {
        let mut solution = 0;
        solution += filled; // Count the first grid

        // Print the grid size for debugging
        println!("Grid size: {}x{}", grid.len(), grid[0].len()); // 131x131
        let middle = grid.len() / 2;

        // Whats the number of filled cells by parity on one board?
        // let (par1, par2) = floodfill(&grid, vec![(0, 0, 0)]);
        let (par1, par2) = floodfill(&part2grid, vec![(start.0, start.1, 0)], usize::MAX);
        println!("Parity 1: {}, Parity 2: {}", par1, par2);

        // calculate the parity for starting at the south middle (for all grids directly above the starting grid)
        let (par1s, par2s) = floodfill(&part2grid, vec![(grid.len() - 1, middle, 0)], usize::MAX);

        // calculate the parity for starting at the north middle (for all grids directly below the starting grid)
        let (par1n, par2n) = floodfill(&part2grid, vec![(0, middle, 0)], usize::MAX);

        // calculate the parity for starting at the east middle (for all grids directly left of the starting grid)
        let (par1e, par2e) =
            floodfill(&part2grid, vec![(middle, grid[0].len() - 1, 0)], usize::MAX);

        // calculate the parity for starting at the west middle (for all grids directly right of the starting grid)
        let (par1w, par2w) = floodfill(&part2grid, vec![(middle, 0, 0)], usize::MAX);

        // There are also the four quadrants, which get filled differently, because ie the NW quadrant is filled from the south east corner.

        // calculate the parity for starting at the north west corner (for all grids in the south east quadrant)
        let (par1nw, par2nw) = floodfill(&part2grid, vec![(0, 0, 0)], usize::MAX);

        // calculate the parity for starting at the north east corner (for all grids in the south west quadrant)
        let (par1ne, par2ne) = floodfill(&part2grid, vec![(0, grid[0].len() - 1, 0)], usize::MAX);

        // calculate the parity for starting at the south west corner (for all grids in the north east quadrant)
        let (par1sw, par2sw) = floodfill(&part2grid, vec![(grid.len() - 1, 0, 0)], usize::MAX);

        // calculate the parity for starting at the south east corner (for all grids in the north west quadrant)
        let (par1se, par2se) = floodfill(
            &part2grid,
            vec![(grid.len() - 1, grid[0].len() - 1, 0)],
            usize::MAX,
        );

        // Now, these values can be used to simulate the looping grid filling.
        // The directional grids (n, s, e, w) start filling after middle+1 steps, the diagonal grids (nw, ne, sw, se) start filling after 2*middle+1 steps.

        // Calculate how many times the max steps fit into the grid size * 2 (to account for the switching of parity)
        let max_steps_fit_directional = (max_steps - (middle + 1)) / (2 * grid.len());
        let max_steps_fit_diagonal = (max_steps - (2 * middle + 1)) / (2 * grid.len());

        // Add the directional and diagonal grids to the solution
        if max_steps_fit_directional > 0 {
            solution += max_steps_fit_directional
                * (par1s + par2s + par1n + par2n + par1e + par2e + par1w + par2w);
        }
        if max_steps_fit_diagonal > 0 {
            // There are quadratically many diagonal grids. They are aranged in a triangle, so there are (n+1)n/2 grids in the nth iteration.
            let diag_grids = (max_steps_fit_diagonal + 1) * max_steps_fit_diagonal;
            // let diag_grids = (max_steps_fit_diagonal + 1) * max_steps_fit_diagonal / 2;
            solution += diag_grids
                * (par1nw + par2nw + par1ne + par2ne + par1sw + par2sw + par1se + par2se);
        }

        let remaining_steps_directional =
            max_steps - (max_steps_fit_directional * (2 * grid.len()));
        let remaining_steps_diagonal = max_steps - (max_steps_fit_diagonal * (2 * grid.len()));

        dbg!(solution); /////////////// needs to be 599763.......

        // println!("Remaining steps directional: {}", remaining_steps_directional);

        // Simulate the remaining steps

        // calculate the parity for starting at the south middle (for all grids directly above the starting grid)
        let (par1s, par2s) = floodfill(
            &part2grid,
            vec![(grid.len() - 1, middle, 0)],
            remaining_steps_directional,
        );

        // calculate the parity for starting at the north middle (for all grids directly below the starting grid)
        let (par1n, par2n) = floodfill(
            &part2grid,
            vec![(0, middle, 0)],
            remaining_steps_directional,
        );

        // calculate the parity for starting at the east middle (for all grids directly left of the starting grid)
        let (par1e, par2e) = floodfill(
            &part2grid,
            vec![(middle, grid[0].len() - 1, 0)],
            remaining_steps_directional,
        );

        // calculate the parity for starting at the west middle (for all grids directly right of the starting grid)
        let (par1w, par2w) = floodfill(
            &part2grid,
            vec![(middle, 0, 0)],
            remaining_steps_directional,
        );

        // I think the diagonal grids can just do the same?

        // calculate the parity for starting at the north west corner (for all grids in the south east quadrant)
        let (par1nw, par2nw) = floodfill(&part2grid, vec![(0, 0, 0)], remaining_steps_diagonal);

        // calculate the parity for starting at the north east corner (for all grids in the south west quadrant)
        let (par1ne, par2ne) = floodfill(
            &part2grid,
            vec![(0, grid[0].len() - 1, 0)],
            remaining_steps_diagonal,
        );

        // calculate the parity for starting at the south west corner (for all grids in the north east quadrant)
        let (par1sw, par2sw) = floodfill(
            &part2grid,
            vec![(grid.len() - 1, 0, 0)],
            remaining_steps_diagonal,
        );

        // calculate the parity for starting at the south east corner (for all grids in the north west quadrant)
        let (par1se, par2se) = floodfill(
            &part2grid,
            vec![(grid.len() - 1, grid[0].len() - 1, 0)],
            remaining_steps_diagonal,
        );

        // There also might be a second set of incomplete diagonal grids.
        if remaining_steps_diagonal > grid.len() {
            // calculate the parity for starting at the north west corner (for all grids in the south east quadrant)
            let (par1nw, par2nw) = floodfill(
                &part2grid,
                vec![(0, 0, 0)],
                remaining_steps_diagonal - grid.len(),
            );

            // calculate the parity for starting at the north east corner (for all grids in the south west quadrant)
            let (par1ne, par2ne) = floodfill(
                &part2grid,
                vec![(0, grid[0].len() - 1, 0)],
                remaining_steps_diagonal - grid.len(),
            );

            // calculate the parity for starting at the south west corner (for all grids in the north east quadrant)
            let (par1sw, par2sw) = floodfill(
                &part2grid,
                vec![(grid.len() - 1, 0, 0)],
                remaining_steps_diagonal - grid.len(),
            );

            // calculate the parity for starting at the south east corner (for all grids in the north west quadrant)
            let (par1se, par2se) = floodfill(
                &part2grid,
                vec![(grid.len() - 1, grid[0].len() - 1, 0)],
                remaining_steps_diagonal - grid.len(),
            );

            // Add the second set of diagonal grids to the solution
            solution += max_steps_fit_diagonal*(par1nw + par1ne + par1sw + par1se); // TODO: is that the correct parity?
            // solution += (par1nw + par1ne + par1sw + par1se); // TODO: is that the correct parity?
                                                           // solution += max_steps_fit_diagonal*(par2nw + par2ne + par2sw + par2se); // TODO: is that the correct parity?
        }

        // Add the remaining steps to the solution, but only the correct parity
        solution += par1s + par1n + par1e + par1w + max_steps_fit_diagonal*(par2nw + par2ne + par2sw + par2se); // TODO: is that the correct parity?
                                                                                       // solution += par2s + par2n + par2e + par2w + par1nw + par1ne + par1sw + par1se; // TODO: is that the correct parity? // It should be...

        println!("Part 2: {}", solution);
    }
    // 11858728186 is too low
    // 11858729163 would be with >= in the floodfill
    // 11858728391 with flipped parity
    // Corrected the double incomplete diagonal grid stuff:
    // 11858732727 with > and 2-parity at the cardinal directions: too low
    // 11858733387 with >= and 1-parity at the cardinal directions: too low
    // (Darn, I forgot about there being quadratically many diagonal grids!)
    // 299883027146847 with > and 2-parity at the cardinal directions: too low
    // 299883027147679 with >=
    // 299883027147013 with 1-parity
    // 299883027147507 with both; incorrect.
    // Fixing grid numbers ...
    // 599760124979340
    // 599760124979145 with 1-parity
    // 599763086592585 with single diag grid end fix
    // 599765702785865 with double diag grid end fix

    // Alternate way: find how many cells you can reach with the remainder first.
    // Then calculate how many with the remainder + grid.len() and then with remainder + 2*grid.len().

    // let remainder = max_steps % grid.len();
    let remainder = grid.len();
    dbg!(remainder);

    // floodfill for the remainder starting at the start
    let (par1, par2) = floodfill(&part2grid, vec![(start.0, start.1, 0)], remainder + 1);
    // dbg!(par1, par2); // 986?
    // println!("({},{})", remainder, par1);

    let p1 = par1;

    let n = 11;
    // Create a nxn grid of grids
    let mut supergrid: Vec<Vec<char>> = Vec::new(); // a 2d grid that contains the original grid n^2 times
    for _ in 0..n {
        let gridclone = part2grid.clone();
        const EMPTY_VEC: Vec<char> = Vec::new();
        let mut row: Vec<Vec<char>> = vec![EMPTY_VEC.clone(); gridclone.len()];
        for _ in 0..n {
            for i in 0..gridclone.len() {
                row[i].append(&mut gridclone[i].clone());
            }
        }
        for i in 0..row.len() {
            supergrid.push(row[i].clone());
        }
    }
    dbg!(supergrid.len(), supergrid[0].len());

    // Debug print grif
    // for row in supergrid.iter() {
    //     for col in row.iter() {
    //         print!("{}", col);
    //     }
    //     println!();
    // }

    let nmiddle = n / 2;
    // Floodfill for the remainder + grid.len() starting at the start + nmiddle * grid.len()
    let (par1, par2) = floodfill(
        &supergrid,
        vec![(
            start.0 + nmiddle * grid.len(),
            start.1 + nmiddle * grid.len(),
            0,
        )],
        grid.len() + remainder,
    );
    // dbg!(par1, par2);
    // println!("({},{})", remainder + grid.len(), par1);
    let p2 = par1;

    // Floodfill for the remainder + 2 * grid.len() starting at the start + nmiddle * grid.len()
    let (par3, par4) = floodfill(
        &supergrid,
        vec![(
            start.0 + nmiddle * grid.len(),
            start.1 + nmiddle * grid.len(),
            0,
        )],
        2 * grid.len() + remainder,
    );
    // dbg!(par3, par4);
    // println!("({},{})", remainder + 2 * grid.len(), par3);
    let p3 = par3;

    // Floodfill for the remainder + 3 * grid.len() starting at the start + nmiddle * grid.len()
    let (par5, par6) = floodfill(
        &supergrid,
        vec![(
            start.0 + nmiddle * grid.len(),
            start.1 + nmiddle * grid.len(),
            0,
        )],
        3 * grid.len() + remainder,
    );
    // dbg!(par5, par6);
    // println!("({},{})", remainder + 3 * grid.len(), par5);
    let p4 = par5;

    // Floodfill for the remainder + 4 * grid.len() starting at the start + nmiddle * grid.len()
    let (par7, par8) = floodfill(
        &supergrid,
        vec![(
            start.0 + nmiddle * grid.len(),
            start.1 + nmiddle * grid.len(),
            0,
        )],
        4 * grid.len() + remainder,
    );
    // dbg!(par7, par8);
    let p5 = par7;

    println!(
        "Part 2 coordinates: ({},{}), ({},{}), ({},{}), ({},{}), ({},{})",
        remainder,
        p1,
        remainder + grid.len(),
        p2,
        remainder + 2 * grid.len(),
        p3,
        remainder + 3 * grid.len(),
        p4,
        remainder + 4 * grid.len(),
        p5
    );

    // same, but with (0, y) instead
    println!(
        "Part 2 coordinates: (1,{}), (2,{}), (3,{}), (4,{}), (5,{})",
        p1, p2, p3, p4, p5
    );

    // same, but with (0, diff) instead
    println!(
        "Part 2 coordinates: (1,{}), (2,{}), (3,{}), (4,{}), (5,{})",
        p1,
        p2 - p1,
        p3 - p2,
        p4 - p3,
        p5 - p4
    );

    // Test number 2

    // Interpolate by
}

// Floodfill the given grid, starting at the given points with iterations
fn floodfill(
    grid: &Vec<Vec<char>>,
    start: Vec<(usize, usize, usize)>,
    stopiterations: usize,
) -> (usize, usize) {
    let mut count1 = 0; // Parity: O or H
    let mut count2 = 0;

    let mut grid = grid.clone();

    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new(); // (i, j, depth)
    let mut appendqueue = start.clone().into();
    queue.append(&mut appendqueue);

    let mut westqueue: Vec<(usize, usize, usize)> = Vec::new(); // For the next grid west
    let mut eastqueue: Vec<(usize, usize, usize)> = Vec::new(); // For the next grid east
    let mut northqueue: Vec<(usize, usize, usize)> = Vec::new(); // For the next grid north
    let mut southqueue: Vec<(usize, usize, usize)> = Vec::new(); // For the next grid south

    while !queue.is_empty() {
        let (i, j, u) = queue.pop_front().unwrap();
        if stopiterations != usize::MAX && u > stopiterations {
            continue;
        }
        if grid[i][j] == '.' {
            // Partity: O or H
            grid[i][j] = if u % 2 == 0 {
                count1 += 1;
                'O'
            } else {
                count2 += 1;
                'H'
            };
            if i > 0 {
                queue.push_back((i - 1, j, u + 1));
            } else {
                northqueue.push((grid.len() + i - 1, j, u + 1));
            }

            if i < grid.len() - 1 {
                queue.push_back((i + 1, j, u + 1));
            } else {
                southqueue.push((i + 1 - grid.len(), j, u + 1));
            }

            if j > 0 {
                queue.push_back((i, j - 1, u + 1));
            } else {
                westqueue.push((i, grid[i].len() + j - 1, u + 1));
            }

            if j < grid[i].len() - 1 {
                queue.push_back((i, j + 1, u + 1));
            } else {
                eastqueue.push((i, j + 1 - grid[i].len(), u + 1));
            }
        }
    }

    // Debug print grif
    // for (i, row) in grid.iter().enumerate() {
    //     for (j, col) in row.iter().enumerate() {
    //         // Print S at the start
    //         if start.iter().any(|(x, y, _)| *x == i && *y == j) {
    //             print!("S");
    //         } else {
    //             print!("{}", col);
    //         }
    //     }
    //     println!();
    // }
    // println!();
    // println!();
    // println!();
    // println!();
    // println!();

    return (count1, count2);
}
