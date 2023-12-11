use std::collections::HashSet;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Vert,
    Horiz,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Continue, // Only Part 2
    Start,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn main() {

    let text = include_str!("TEMP.txt");
    // let text = include_str!("TEMP2.txt");
    // let text = include_str!("TEMP3.txt");

    // Parse the 2D map
    let mut map: Vec<Vec<Tile>> = Vec::new();

    for line in text.lines() {
        let mut row: Vec<Tile> = Vec::new();
        for c in line.chars() {
            match c {
                '|' => row.push(Tile::Vert),
                '-' => row.push(Tile::Horiz),
                'L' => row.push(Tile::NE),
                'J' => row.push(Tile::NW),
                '7' => row.push(Tile::SW),
                'F' => row.push(Tile::SE),
                '.' => row.push(Tile::Ground),
                'S' => row.push(Tile::Start),
                a => panic!("Unknown character: {}", a),
            }
        }
        map.push(row);
    }

    // XY coordinates of the start tile

    let start_pos:(usize, usize) = map.iter().enumerate().find_map(|(y, row)| {
        row.iter().enumerate().find_map(|(x, tile)| {
            if *tile == Tile::Start {
                Some((x, y))
            } else {
                None
            }
        })
    }).unwrap();

    println!("Start position: {:?}", start_pos);

    let (_, looping) = find_loop_and_set(&map, start_pos, Direction::South);

    println!("Looping: {:?}", looping.is_some());
    println!("Looping length: {:?}", looping.unwrap().len());


    // Part 2:
    // Find the area incloses within the loop.
    
    // This will be done with floodfill, but "doubly inclosed" areas should not be counted. 

    // To solve this, we first "inflate" the map, that is, make it twice the size in each direction.
    // The map will also have an outer border of Continue tiles, which will be used to distribute the floodfill.

    // To inflate the map, we first create a new based on the old one, but with the new size.

    let mut inflated_map = Vec::new();

    // First row, full of Continue tiles
    let continue_row = vec![Tile::Continue; map[0].len() * 2 + 1];
    inflated_map.push(continue_row.clone());

    for i in 0..map.len() {
        let mut row = Vec::new();
        row.push(Tile::Continue);
        for j in 0..map[0].len() {
            row.push(map[i][j]);
            row.push(Tile::Continue);
        }
        inflated_map.push(row);
        inflated_map.push(continue_row.clone());
    }

    // Find the new start position
    let start_pos = (start_pos.0 * 2 + 1, start_pos.1 * 2 + 1);
    // dbg!(inflated_map[start_pos.1][start_pos.0]); //Yep, it's Start

    // Calculate the new loop
    let (floodfill_map, looping) = find_loop_and_set(&inflated_map, start_pos, Direction::South);

    println!("Looping: {:?}", looping.is_some());
    let looping = looping.unwrap();
    // Now floodfill the map, starting from 0,0.

    let mut floodfilled = HashSet::new();

    let mut floodfill_queue = Vec::new();
    floodfill_queue.push((0, 0));

    while !floodfill_queue.is_empty() {
        let pos = floodfill_queue.pop().unwrap();

        // Check if the position is already filled
        if floodfilled.contains(&pos) {
            continue;
        }

        // Check if the position is within the loop
        if looping.contains(&pos) {
            continue;
        }

        // Just to be sure, check if the position is within the map
        if pos.0 >= floodfill_map[0].len() || pos.1 >= floodfill_map.len() {
            continue;
        }

        // Add the position to the floodfilled map
        floodfilled.insert(pos);

        // Add the adjacent tiles to the queue
        floodfill_queue.push((pos.0 + 1, pos.1));
        let minus = pos.0.checked_sub(1);
        if minus.is_some() {
            floodfill_queue.push((minus.unwrap(), pos.1));
        }
        floodfill_queue.push((pos.0, pos.1 + 1));
        let minus = pos.1.checked_sub(1);
        if minus.is_some() {
            floodfill_queue.push((pos.0, minus.unwrap()));
        }
    }
    println!("Done floodfilling; {}", floodfilled.len());

    // Count how many not-filled, non-looping tiles there are
    let mut count = 0;
    for i in 0..floodfill_map.len() {
        for j in 0..floodfill_map[0].len() {
            // print!("{};{}",i,j);
            if !floodfilled.contains(&(j, i)) && !looping.contains(&(j, i)) && floodfill_map[i][j] != Tile::Continue{
                count += 1;
            }
        }
    }

    println!("Count: {}", count);

}
// Tried solutions:
// 13692; 6981 too high??
// FAIL... 6.846???
// I miscalculated..... Yay?

/// Find the loop around the map
/// Which starts at the start position and ends at the start position
/// Also sets any Continue tiles to the correct tile
fn find_loop_and_set(map: &Vec<Vec<Tile>>, start_pos: (usize, usize), start_dir: Direction) -> (Vec<Vec<Tile>>, Option<Vec<(usize, usize)>>) {

    let mut map = map.clone(); // New map, so we can set Continue tiles

    let mut pos = start_pos;
    let mut dir = start_dir;

    let mut loop_pos = Vec::new();

    loop {
        loop_pos.push(pos);

        // Find the next tile
        let next_pos = match dir {
            Direction::North => (pos.0, pos.1 - 1),
            Direction::South => (pos.0, pos.1 + 1),
            Direction::West => (pos.0 - 1, pos.1),
            Direction::East => (pos.0 + 1, pos.1),
        };

        if next_pos.0 >= map[0].len() || next_pos.1 >= map.len() {
            return (map, None);
        }

        // Check if the next tile is a corner
        let next_tile = map[next_pos.1][next_pos.0];
        dir = match next_tile {
            Tile::NE => match dir {
                Direction::South => Direction::East,
                Direction::West => Direction::North,
                _ => panic!("Invalid direction"),
            },
            Tile::NW => match dir {
                Direction::South => Direction::West,
                Direction::East => Direction::North,
                _ => panic!("Invalid direction"),
            },
            Tile::SW => match dir {
                Direction::North => Direction::West,
                Direction::East => Direction::South,
                _ => panic!("Invalid direction"),
            },
            Tile::SE => match dir {
                Direction::North => Direction::East,
                Direction::West => Direction::South,
                _ => panic!("Invalid direction"),
            },
            Tile::Ground => panic!("Invalid direction, moved onto ground"),
            Tile::Continue => {
                // Set the Continue tile to the correct tile
                map[next_pos.1][next_pos.0] = match dir {
                    Direction::North => Tile::Vert,
                    Direction::South => Tile::Vert,
                    Direction::West => Tile::Horiz,
                    Direction::East => Tile::Horiz,
                };
                dir
            }
            _ => dir,
        };

        // Check if the loop is complete
        if next_pos == start_pos {
            break;
        }

        pos = next_pos;
    }

    (map, Some(loop_pos))



}