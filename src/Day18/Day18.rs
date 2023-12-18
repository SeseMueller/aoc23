use std::collections::VecDeque;

fn main() {
    let text = include_str!("TEMP.txt");
    // let text = include_str!("TEMP2.txt");
    // let text = include_str!("TEMP3.txt");

    // Read the Direction and number of steps

    let movement = text.trim().split("\n").map(|x| {
        let mut iter = x.split_whitespace();
        let direction = iter.next().unwrap();
        let steps = iter.next().unwrap().parse::<i32>().unwrap();
        // (direction, steps)

        let hex = iter.next().unwrap();
        let hex = hex.trim_start_matches("(#").trim_end_matches(")");
        // Parse hex
        let last = match hex[hex.len() - 1..].parse::<i64>().unwrap() {
            0 => 'R',
            1 => 'D',
            2 => 'L',
            3 => 'U',
            _ => panic!("Invalid hex"),
        };
        let hex = i64::from_str_radix(&hex[..(hex.len() - 1)], 16).unwrap();
        (last, hex)
    });

    // Convert to a vector of (x, y) coordinates
    println!("Converting to coords");

    // let mut coords = vec![(0, 0)];
    let mut coords = vec![];

    let mut curr_pos = (0i64, 0);

    let _: Vec<_> = movement
        .map(|(direction, steps)| match direction {
            'R' => {
                for _ in 1..steps + 1 {
                    coords.push((curr_pos, direction));
                    curr_pos.1 += 1;
                }
            }
            'L' => {
                for _ in 1..steps + 1 {
                    if coords.last().unwrap_or(&((0, 0), '#')).1 == 'D' {
                        // Edge case (literally)
                        coords.push((curr_pos, 'D'));
                    } else {
                        coords.push((curr_pos, direction));
                    }
                    curr_pos.1 -= 1;
                }
            }
            'U' => {
                for _ in 1..steps + 1 {
                    coords.push((curr_pos, direction));
                    curr_pos.0 += 1;
                }
            }
            'D' => {
                for _ in 1..steps + 1 {
                    coords.push((curr_pos, direction));
                    curr_pos.0 -= 1;
                }
            }
            _ => panic!("Invalid direction"),
        })
        .collect();

    println!("Converting to coords done");

    // HOLY SHIT THAT'S TEN GIGABYTES OF COORDS

    // dbg!(coords.len());

    // Find the interior area by flood filling

    let min_x = coords.iter().map(|(x, _)| x.0).min().unwrap();
    let max_x = coords.iter().map(|(x, _)| x.0).max().unwrap();
    let min_y = coords.iter().map(|(y, _)| y.1).min().unwrap();
    let max_y = coords.iter().map(|(y, _)| y.1).max().unwrap();

    dbg!(min_x, max_x, min_y, max_y);

    // let mut area = vec![vec![0; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];

    // // Construct grid from coords
    // for ((x, y), _) in coords.clone() {
    //     area[(y - min_y) as usize][(x - min_x) as usize] = 1;
    // }

    // // Find first interiour in second row
    // let mut first_interior = None;
    // for (i, col) in area[1].iter().enumerate() {
    //     if *col == 1 {
    //         first_interior = Some(i);
    //         break;
    //     }
    // }
    // let first_interior = first_interior.unwrap();

    // // Pad the grid with a border of 0s
    // for row in &mut area {
    //     row.insert(0, 0);
    //     row.push(0);
    // }
    // area.insert(0, vec![0; area[0].len()]);
    // area.push(vec![0; area[0].len()]);

    // // Visualize grid
    // // for row in area.clone() {
    // //     for col in row {
    // //         print!("{}", col);
    // //     }
    // //     println!();
    // // }

    // // Flood fill
    // let mut queue = vec![(2, first_interior + 2)];

    // let mut size = 0;

    // while !queue.is_empty() {
    //     let (y, x) = queue.pop().unwrap();

    //     if area[y][x] == 2 {
    //         continue;
    //     }

    //     size += 1;

    //     area[y][x] = 2;

    //     if area[y - 1][x] == 0 {
    //         queue.push((y - 1, x));
    //     }
    //     if area[y + 1][x] == 0 {
    //         queue.push((y + 1, x));
    //     }
    //     if area[y][x - 1] == 0 {
    //         queue.push((y, x - 1));
    //     }
    //     if area[y][x + 1] == 0 {
    //         queue.push((y, x + 1));
    //     }
    // }

    // println!("Area size: {}", size + coords.len());

    // Visualize coords
    // for x in min_x..max_x + 1 {
    //     for y in min_y..max_y + 1 {
    //         if coords.contains(&((x, y), "R")) {
    //             print!("R");
    //         } else if coords.contains(&((x, y), "L")) {
    //             print!("L");
    //         } else if coords.contains(&((x, y), "U")) {
    //             print!("U");
    //         } else if coords.contains(&((x, y), "D")) {
    //             print!("D");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    // Now do it without flood filling

    dbg!(coords.len());
    let mut size2: i64 = 0;

    let mut coords2 = coords.clone();

    println!("Sorting coords");
    // Go over every row of the coords
    coords2.sort();

    let mut coords2: VecDeque<_> = coords2.clone().into(); // For front pop

    let mut last_percent = -1;

    println!("Finding intersections...");
    for x in min_x..max_x + 1 {
        let mut intersects = vec![];

        // Progress bar
        let percent = (x - min_x) * 100 / (max_x - min_x);
        // dbg!(percent);
        if percent != last_percent {
            println!("\r{}%", percent);
            last_percent = percent;
        }

        loop {
            let result = coords2.pop_front(); //TODO: check if empty
            if result.is_none() {
                break;
            }
            let ((x2, y2), s) = result.unwrap();
            if x2 == x {
                intersects.push((y2, s));
            } else {
                coords2.push_front(((x2, y2), s));
                break;
            }
        }

        for inter in intersects.windows(2) {
            let firstletter = inter[0].1;
            let secondletter = inter[1].1;
            if (firstletter == 'L' || firstletter == 'U')
                && (secondletter == 'R' || secondletter == 'D')
            {
                size2 += inter[1].0 - inter[0].0 - 1;
            }
        }
    }

    println!("\nArea size2: {}", size2 + coords.len() as i64);
}
