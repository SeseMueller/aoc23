use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let text = include_str!("TEMP.txt");
    // let text = include_str!("TEMP2.txt");
    // let text = include_str!("TEMP3.txt");

    // This time, it's light and mirrors.
    // . is empty space and -/|\ are mirrors.
    // Light only moves orthogonally and is split when hitting - and |.

    let grid: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();
    
    // Part 1
    let lights = generate_lights(&grid, ((0, 0), Direction::Right));

    println!("Number of lights: {}", lights.iter().unique_by(|x| (x.0,x.1)).collect::<Vec<_>>().len());
    // println!("Lights: {:?}", lights);

    // pretty_print(grid, lights)

    // Part 2

    // The start can be anywhere on the edge now ....

    let mut max_illuminated = 0;

    // Top edge
    for x in 0..grid[0].len() {
        let lights = generate_lights(&grid, ((x, 0), Direction::Down));
        max_illuminated = max_illuminated.max(lights.iter().unique_by(|x| (x.0,x.1)).collect::<Vec<_>>().len());
    }
    dbg!(max_illuminated);
    
    // Bottom edge
    for x in 0..grid[0].len() {
        let lights = generate_lights(&grid, ((x, grid.len() - 1), Direction::Up));
        max_illuminated = max_illuminated.max(lights.iter().unique_by(|x| (x.0,x.1)).collect::<Vec<_>>().len());
    }
    dbg!(max_illuminated);
    
    // Left edge
    for y in 0..grid.len() {
        let lights = generate_lights(&grid, ((0, y), Direction::Right));
        max_illuminated = max_illuminated.max(lights.iter().unique_by(|x| (x.0,x.1)).collect::<Vec<_>>().len());
    }
    dbg!(max_illuminated);
    
    // Right edge
    for y in 0..grid.len() {
        let lights = generate_lights(&grid, ((grid[0].len() - 1, y), Direction::Left));
        max_illuminated = max_illuminated.max(lights.iter().unique_by(|x| (x.0,x.1)).collect::<Vec<_>>().len());
    }

    println!("Max illuminated: {}", max_illuminated);

}

// Generate lights
fn generate_lights(grid: &Vec<Vec<char>>, start: ((usize, usize), Direction)) -> Vec<(usize, usize, Direction)>{
    let mut lights: Vec<(usize, usize, Direction)> = Vec::new();

    let mut todo = vec![start];

    while let Some(((x, y), dir)) = todo.pop() {
        let c = grid[y][x];

        // dbg!(todo.clone(), (x, y, dir), c);

        // If it already has a light, then we're done
        if lights.contains(&(x, y, dir)) {
            continue;
        }

        // Add to lights
        lights.push((x, y, dir));
        match c {
            '.' => {
                // Just keep moving
                let (dx, dy) = match dir {
                    Direction::Up => (0, -1),
                    Direction::Down => (0, 1),
                    Direction::Left => (-1, 0),
                    Direction::Right => (1, 0),
                };
                let (nx, ny) = (x as isize + dx, y as isize + dy);
                if nx < 0 || ny < 0 {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if ny >= grid.len() || nx >= grid[ny].len() {
                    continue;
                }
                todo.push(((nx, ny), dir));
            }
            '-' => {
                // If going up or down, also split (left and right)
                match dir {
                    Direction::Up | Direction::Down => {
                        let left_new = ((x as i32 -1, y), Direction::Left);
                        let right_new = ((x + 1, y), Direction::Right);

                        if left_new.0 .0 >= 0 {
                            todo.push(((x - 1, y), Direction::Left));
                        }

                        if right_new.0 .0 < grid.len() {
                            todo.push(right_new);
                        }
                    }
                    Direction::Left | Direction::Right => {
                        // Just keep moving
                        let (dx, dy) = match dir {
                            Direction::Up => (0, -1),
                            Direction::Down => (0, 1),
                            Direction::Left => (-1, 0),
                            Direction::Right => (1, 0),
                        };

                        let (nx, ny) = (x as isize + dx, y as isize + dy);
                        if nx < 0 || ny < 0 {
                            continue;
                        }
                        let (nx, ny) = (nx as usize, ny as usize);
                        if ny >= grid.len() || nx >= grid[ny].len(){
                            continue;
                        }
                        todo.push(((nx, ny), dir));
                    }
                }
            }

            '|' => {
                // If going left or right, also split (up and down)
                match dir {
                    Direction::Left | Direction::Right => {
                        let up_new = ((x, y as i32 - 1), Direction::Up);
                        let down_new = ((x, y + 1), Direction::Down);

                        if up_new.0 .1 >= 0 {
                            todo.push(((x, y - 1), Direction::Up));
                        }

                        if down_new.0 .1 < grid[y].len() {
                            todo.push(down_new);
                        }
                    }
                    Direction::Up | Direction::Down => {
                        // Just keep moving
                        let (dx, dy) = match dir {
                            Direction::Up => (0, -1),
                            Direction::Down => (0, 1),
                            Direction::Left => (-1, 0),
                            Direction::Right => (1, 0),
                        };

                        let (nx, ny) = (x as isize + dx, y as isize + dy);
                        if nx < 0 || ny < 0 {
                            continue;
                        }
                        let (nx, ny) = (nx as usize, ny as usize);
                        if ny >= grid.len() || nx >= grid[ny].len() {
                            continue;
                        }
                        todo.push(((nx, ny), dir));
                    }
                }
            }
            '/' => {
                // Reflect
                let new_dir = match dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };

                let (dx, dy) = match new_dir {
                    Direction::Up => (0, -1),
                    Direction::Down => (0, 1),
                    Direction::Left => (-1, 0),
                    Direction::Right => (1, 0),
                };

                let (nx, ny) = (x as isize + dx, y as isize + dy);
                if nx < 0 || ny < 0 {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if ny >= grid.len() || nx >= grid[ny].len(){
                    continue;
                }
                todo.push(((nx, ny), new_dir));
            }

            '\\' => {
                // Reflect
                let new_dir = match dir {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };

                let (dx, dy) = match new_dir {
                    Direction::Up => (0, -1),
                    Direction::Down => (0, 1),
                    Direction::Left => (-1, 0),
                    Direction::Right => (1, 0),
                };

                let (nx, ny) = (x as isize + dx, y as isize + dy);
                if nx < 0 || ny < 0 {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if ny >= grid.len() || nx >= grid[ny].len() {
                    continue;
                }
                todo.push(((nx, ny), new_dir));
            }
            _ => {
                panic!("Unknown character: {}", c);
            }
        }
    }
    return lights;
}


// Pretty prints the grid and the lights
fn pretty_print(grid: Vec<Vec<char>>, lights: Vec<(usize, usize, Direction)>) {
    // if . print light, otherwise print grid
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '.' {
                let found = lights.iter().filter(|&&(lx, ly, _)| lx == x && ly == y).collect::<Vec<_>>();
                if found.len() == 1 {
                    let dir = found[0].2;
                    let c = match dir {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    };
                    print!("{}", c);
                } else if found.len() > 1{
                    print!("{}", found.len());
                } else {
                    print!(".");
                }
            } else {
                print!("{}", grid[y][x]);
            }
        }
        println!();
    }

}