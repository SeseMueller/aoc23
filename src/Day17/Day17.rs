use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    cost: i32,
    position: (usize, usize),
    path: Vec<(usize, usize)>,
    direction: Direction,
    curr_ran_distance: i32,
}

// Sort by cost, then by position
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.curr_ran_distance.cmp(&self.curr_ran_distance))
            .then_with(|| {
                (self.position.0 + self.position.1).cmp(&(other.position.0 + other.position.1))
            })
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            other
                .cost
                .cmp(&self.cost)
                .then_with(|| other.curr_ran_distance.cmp(&self.curr_ran_distance))
                .then_with(|| {
                    (self.position.0 + self.position.1).cmp(&(other.position.0 + other.position.1))
                }),
        )
    }
}

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

    // A Grid of numbers; minimize the sum of the numbers in the path from 0 0 to the bottom right.

    let grid: Vec<Vec<i32>> = text
        .lines()
        .map(|l| {
            l.chars()
                .map(|s| s.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    // Part 1
    //Just do A* with a heuristic of the manhattan distance to the bottom right.
    // Also note that you can move at most three blocks in one direction before you have to change direction.

    let mut open_set = BinaryHeap::new();
    // let mut open_set = HashSet::new();

    let mut sol_state = vec![];

    open_set.push(State {
        cost: 0,
        position: (0, 0),
        path: vec![],
        direction: Direction::Right,
        curr_ran_distance: 0,
    });

    let mut closed_set: HashMap<(usize, usize), Vec<State>> = HashMap::new();
    let mut max_dist = 0;
    let mut min_tot_cost = 1000000;

    while let Some(State {
        cost,
        position,
        path,
        direction,
        curr_ran_distance,
    }) = open_set.pop()
    {
        if position == (grid[0].len() - 1, grid.len() - 1) {
            // println!("Path: {:?}", path);
            println!("Cost: {}", cost);
            sol_state.push(State {
                cost,
                position,
                path,
                direction,
                curr_ran_distance,
            });
            break;
        }

        if closed_set.contains_key(&position) {
            let old_state: Vec<State> = closed_set.get(&position).unwrap().clone();
            // if   (old_state.cost <= cost && (old_state.curr_ran_distance <= curr_ran_distance && old_state.direction == direction)) {
            // if old_state.cost <= cost && old_state.curr_ran_distance <= curr_ran_distance && (old_state.direction == direction || old_state.curr_ran_distance < 0) {
            // if old_state.cost <= cost && old_state.curr_ran_distance <= curr_ran_distance && (old_state.direction == direction || curr_ran_distance < 1) {
            // if (old_state.cost <= cost && old_state.curr_ran_distance <= curr_ran_distance && (old_state.direction == direction || curr_ran_distance < 0)) || old_state.cost < cost - 27{
            // if ((old_state.cost <= cost - 9*(old_state.curr_ran_distance - curr_ran_distance)) || (old_state.cost <= cost && old_state.curr_ran_distance <= curr_ran_distance)) && old_state.direction == direction {
            // if old_state.iter().any(|os| os.cost <= cost && os.curr_ran_distance <= curr_ran_distance && os.direction == direction ){
            if old_state.iter().any(|os| {
                ((os.cost <= cost && os.curr_ran_distance <= curr_ran_distance)
                    && os.direction == direction)
                    || (os.cost <= cost - 50)
            }) {
                continue;
            }

            closed_set.get_mut(&position).unwrap().push(State {
                cost,
                position,
                path: path.clone(),
                direction,
                curr_ran_distance,
            });
        } else {
            closed_set.insert(
                position,
                vec![State {
                    cost,
                    position,
                    path: path.clone(),
                    direction,
                    curr_ran_distance,
                }],
            );
        }

        // Progress update
        if path.len() > max_dist {
            max_dist = path.len();
            // println!("Path length: {}", max_dist);
        }

        // The total costs are at most cost + 9*(grid.len() + grid[0].len())
        if min_tot_cost > cost + 9 * (grid.len() + grid[0].len() - position.0 - position.1) as i32 {
            min_tot_cost = cost + 9 * (grid.len() + grid[0].len() - position.0 - position.1) as i32;
            // println!("Min cost: {}", min_tot_cost);
        }

        if cost > min_tot_cost {
            continue; //Is this correct?
        }

        // Debug
        if open_set.len() > 100000 {
            // println!("Current position: {:?}", position);
            // Count frequency of each position
            let mut freq: HashMap<(usize, usize), i32> = HashMap::new();
            for s in open_set.iter() {
                *freq.entry(s.position).or_insert(0) += 1;
            }

            for (k, v) in freq.iter() {
                println!("{:?}: {:?}", k, v);
            }

            // Also print all grid 0 0
            for s in open_set.iter() {
                if s.position == (0, 0) {
                    println!("{:?}", s);
                }
            }

            for s in closed_set.values() {
                if s[0].position == (0, 0) {
                    println!("Closed: {:?}", s);
                }
            }
            panic!("Open set too large");
        }

        // println!("OpenLen: {}, ClosedLen: {}, PathLen: {}", open_set.len(), closed_set.len(), path.len());

        let mut new_path = path.clone();
        new_path.push(position);

        for (neighbour, d) in neighbours(position, &grid, direction, true) {
            let crd = if d == direction {
                if curr_ran_distance == 2 {
                    continue;
                }
                curr_ran_distance + 1
            } else {
                0
            };
            open_set.push(State {
                cost: cost + grid[neighbour.1][neighbour.0],
                position: neighbour,
                path: new_path.clone(),
                direction: d,
                curr_ran_distance: crd,
            });
        }
    }

    // let mut sum = grid[grid.len() - 1][grid[0].len() - 1];
    // for (x, y) in &sol_state.first().unwrap().path[1..] {
    //     sum += grid[*y][*x];
    // }

    // println!("Sum: {}", sum);

    // visualize(&grid, sol_state.first().unwrap().clone());

    // Part 2
    // Now the maximum run distance is 10, but the minimum is 4.

    let mut open_set = BinaryHeap::new();

    let mut sol_state = vec![];

    let mut max_dist = 0;

    open_set.push(State {
        cost: 0,
        position: (0, 0),
        path: vec![],
        direction: Direction::Right,
        curr_ran_distance: 0,
    });

    let mut closed_set: HashMap<(usize, usize), Vec<State>> = HashMap::new();

    while let Some(State {
        cost,
        position,
        path,
        direction,
        curr_ran_distance,
    }) = open_set.pop()
    {
        if position == (grid[0].len() - 1, grid.len() - 1) && curr_ran_distance >= 3 {
            // println!("Path: {:?}", path);
            println!("Cost: {}", cost);
            sol_state.push(State {
                cost,
                position,
                path,
                direction,
                curr_ran_distance,
            });
            break;
        }

        if closed_set.contains_key(&position) {
            let old_state: Vec<State> = closed_set.get(&position).unwrap().clone();
            // if   (old_state.cost <= cost && (old_state.curr_ran_distance <= curr_ran_distance && old_state.direction == direction)) {
            // if old_state.cost <= cost && old_state.curr_ran_distance <= curr_ran_distance && (old_state.direction == direction || old_state.curr_ran_distance < 0) {
            // if old_state.cost <= cost && old_state.curr_ran_distance <= curr_ran_distance && (old_state.direction == direction || curr_ran_distance < 1) {
            // if (old_state.cost <= cost && old_state.curr_ran_distance <= curr_ran_distance && (old_state.direction == direction || curr_ran_distance < 0)) || old_state.cost < cost - 27{
            // if ((old_state.cost <= cost - 9*(old_state.curr_ran_distance - curr_ran_distance)) || (old_state.cost <= cost && old_state.curr_ran_distance <= curr_ran_distance)) && old_state.direction == direction {
            // if old_state.iter().any(|os| os.cost <= cost && os.curr_ran_distance <= curr_ran_distance && os.direction == direction ){
            if old_state.iter().any(|os| {
                (os.cost <= cost && os.curr_ran_distance == curr_ran_distance)
                    && os.direction == direction
            }) {
                continue;
            }

            closed_set.get_mut(&position).unwrap().push(State {
                cost,
                position,
                path: path.clone(),
                direction,
                curr_ran_distance,
            });
        } else {
            closed_set.insert(
                position,
                vec![State {
                    cost,
                    position,
                    path: path.clone(),
                    direction,
                    curr_ran_distance,
                }],
            );
        }

        // Progress update
        if path.len() > max_dist {
            max_dist = path.len();
            println!("Path length: {}", max_dist);
        }

        // // The total costs are at most cost + 9*(grid.len() + grid[0].len())
        // if min_tot_cost > cost + 9 * (grid.len() + grid[0].len() - position.0 - position.1) as i32 {
        //     min_tot_cost = cost + 9 * (grid.len() + grid[0].len() - position.0 - position.1) as i32;
        //     // println!("Min cost: {}", min_tot_cost);
        // }

        // if cost > min_tot_cost {
        //     continue; //Is this correct?
        // }

        // Debug
        if open_set.len() > 100000 {
            // println!("Current position: {:?}", position);
            // Count frequency of each position
            let mut freq: HashMap<(usize, usize), i32> = HashMap::new();
            for s in open_set.iter() {
                *freq.entry(s.position).or_insert(0) += 1;
            }

            for (k, v) in freq.iter() {
                println!("{:?}: {:?}", k, v);
            }

            // Also print all grid 0 0
            for s in open_set.iter() {
                if s.position == (0, 0) {
                    println!("{:?}", s);
                }
            }

            for s in closed_set.values() {
                if s[0].position == (0, 0) {
                    println!("Closed: {:?}", s);
                }
            }
            panic!("Open set too large");
        }

        // println!("OpenLen: {}, ClosedLen: {}, PathLen: {}", open_set.len(), closed_set.len(), path.len());

        let mut new_path = path.clone();
        new_path.push(position);

        for (neighbour, d) in neighbours(position, &grid, direction, curr_ran_distance >= 3) {
            let crd = if d == direction {
                if curr_ran_distance == 9 {
                    continue;
                }
                curr_ran_distance + 1
            } else {
                0
            };
            open_set.push(State {
                cost: cost + grid[neighbour.1][neighbour.0],
                position: neighbour,
                path: new_path.clone(),
                direction: d,
                curr_ran_distance: crd,
            });
        }
    }

    // let mut sum = grid[grid.len() - 1][grid[0].len() - 1];
    // for (x, y) in &sol_state.first().unwrap().path[1..] {
    //     sum += grid[*y][*x];
    // }

    // println!("Sum: {}", sum); // Higher than 1048

    // visualize(&grid, sol_state.first().unwrap().clone());
}

fn neighbours(
    position: (usize, usize),
    grid: &Vec<Vec<i32>>,
    curr_dir: Direction,
    can_turn: bool,
) -> Vec<((usize, usize), Direction)> {
    let mut neighbours = vec![];

    if curr_dir != Direction::Right && (can_turn || curr_dir == Direction::Left) {
        if position.0 > 0 {
            neighbours.push(((position.0 - 1, position.1), Direction::Left));
        }
    }

    if curr_dir != Direction::Left && (can_turn || curr_dir == Direction::Right) {
        if position.0 < grid[0].len() - 1 {
            neighbours.push(((position.0 + 1, position.1), Direction::Right));
        }
    }

    if curr_dir != Direction::Down && (can_turn || curr_dir == Direction::Up) {
        if position.1 > 0 {
            neighbours.push(((position.0, position.1 - 1), Direction::Up));
        }
    }

    if curr_dir != Direction::Up && (can_turn || curr_dir == Direction::Down) {
        if position.1 < grid.len() - 1 {
            neighbours.push(((position.0, position.1 + 1), Direction::Down));
        }
    }
    neighbours
}

fn visualize(grid: &Vec<Vec<i32>>, s: State) {
    // // Print <>^v if the state is on the path
    // Print the number if it's not

    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if s.path.contains(&(x, y)) {
                // match s.direction {
                //     Direction::Up => print!("^"),
                //     Direction::Down => print!("v"),
                //     Direction::Left => print!("<"),
                //     Direction::Right => print!(">"),
                // }
                print!(".");
            } else {
                print!("{}", col);
            }
        }
        println!();
    }
}
