use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type Coord = (i32, i32, i32);
type Brick = (Coord, Coord);

fn main() {
    let text = include_str!("TEMP.txt");
    // let text = include_str!("TEMP2.txt");
    // let text = include_str!("TEMP3.txt");

    // Read bricks from lines
    // x1,y1,z1~x2,y2,z2

    let mut bricks: Vec<Brick> = Vec::new();

    for line in text.trim().lines() {
        let (first, second) = line.split("~").collect_tuple().unwrap();
        let (x1, y1, z1) = first.split(",").collect_tuple().unwrap();
        let (x2, y2, z2) = second.split(",").collect_tuple().unwrap();
        let x1 = x1.parse::<i32>().unwrap();
        let y1 = y1.parse::<i32>().unwrap();
        let z1 = z1.parse::<i32>().unwrap();
        let x2 = x2.parse::<i32>().unwrap();
        let y2 = y2.parse::<i32>().unwrap();
        let z2 = z2.parse::<i32>().unwrap();
        bricks.push(((x1, y1, z1), (x2, y2, z2)));
    }

    // for b in &bricks {
    //     print_brick(*b);
    // }
    // println!("");
    // println!("");
    // for b in &give_fallen_bricks(bricks) {
    //     print_brick(*b);
    // }

    // Sort brick by zmax
    bricks.sort_by(|a, b| {
        let az = a.0 .2.max(a.1 .2);
        let bz = b.0 .2.max(b.1 .2);
        let azlen = a.0 .2.min(a.1 .2) - az;
        let bzlen = b.0 .2.min(b.1 .2) - bz;
        // az.cmp(&bz)
        // bz.cmp(&az)
        // az.cmp(&bz).then_with(|| bzlen.cmp(&azlen))
        az.cmp(&bz).then_with(|| azlen.cmp(&bzlen))
    });

    // let base_case = give_fallen_bricks(&bricks); // There's a bug in this function
    let base_case = give_fallen_bricks_old(&bricks);

    // for b in &base_case {
    //     print_brick(*b);
    // }

    let mut p1_bricks: HashMap<Brick, Vec<Brick>> = HashMap::new();

    // // For each brick, try removing it. If the remaining bricks stay at the same location, it is part of part 1
    // for b in bricks.clone() {
    //     let mut remaining_bricks = bricks.clone();
    //     remaining_bricks.retain(|&x| x != b);
    //     let case_without_b = give_fallen_bricks(&remaining_bricks);
    //     // This now doesn't contain b, so we can't just check if it is equal to base_case
    //     if case_without_b.iter().all(|x| base_case.contains(x)) {
    //         p1_bricks.push(b);
    //     }
    // }

    // too slow! Instead, for each brick, construct first which bricks it depends on
    // Then invert and find which brick depends on it

    for b1 in base_case.clone() {
        for b2 in base_case.clone() {
            if b1 == b2 {
                continue;
            }
            if zdistance2(b1, b2).unwrap_or(-1) == 1 {
                // b1 depends on b2
                // p1_bricks.insert(b2, b1);
                p1_bricks.entry(b2).or_insert(Vec::new()).push(b1);
            }
        }
    }

    //DEBUG
    // for (b1, b2) in &p1_bricks {
    //     print_brick(*b1);
    //     for b in b2 {
    //         // print_brick(*b);
    //         // // For Graph viasualization
    //         // println!("{:?} --> {:?}", (b1), (b));
    //     }
    //     // println!("");
    // }

    // print that lenth
    // println!("Part 1: {}", p1_bricks.len()); // 469 is too high?

    // If a brick isn't in there or has more than one dependency, it is part of part 1
    // let p1_sol1 = base_case.clone().into_iter().filter(|x| !p1_bricks.contains_key(x)).count(); // not in there
    // In there but has more than one dependency
    let required_bricks: HashSet<Brick> = p1_bricks
        .iter()
        .filter(|(_, v)| v.len() == 1)
        .map(|(_, v)| v[0])
        .collect();

    println!("Part 1: {}", base_case.len() - required_bricks.len()); // 459 is too low; 461 is right

    // Part2 is not very difficult with the hashmap dependency graph.

    // For each brick, if it were removed, how many other bricks would fall?
    let mut p2_sum = 0;

    // Check each brick, remove it in the hashmap, and then check how many bricks fall
    // To make it easier, put an empty vetor in the hashmap for each brick that doesn't have any dependencies
    let mut p2_map: HashMap<Brick, Vec<Brick>> = p1_bricks.clone();

    for b in base_case.clone() {
        p2_map.entry(b).or_insert(Vec::new());
    }

    for b1 in &base_case {
        // Create a new hashmap without b1
        let mut p2_map_without_b1 = p2_map.clone();
        p2_map_without_b1.remove(b1);

        let mut removed = HashSet::new();
        removed.insert(*b1);

        let mut total_removed = HashSet::new();

        // Until nothing changes, remove all dependencies that have been removed
        let mut something_changed = true;
        while something_changed {
            something_changed = false;
            let prev_len: usize = p2_map_without_b1
                .iter()
                .map(|(_, v)| v.len())
                .sum();

            let prev_map = p2_map_without_b1.clone();

            // DEBUG
            // if b1.0.0 == 0 && b1.0.1 == 1 {
            //     print_brick(*b1);
            //     println!("{:?}", p2_map_without_b1);
            // }

            for key in p2_map_without_b1.clone().keys() {
                let mut new_deps = p2_map_without_b1[key].clone();
                // dbg!(new_deps.clone());
                new_deps.retain(|x| !removed.contains(x));
                // dbg!(new_deps.clone());
                p2_map_without_b1.insert(*key, new_deps);
            }

            // DEBUG
            // if b1.0.0 == 0 && b1.0.1 == 1 {
            //     print_brick(*b1);
            //     println!("{:?}", p2_map_without_b1);
            // }

            total_removed.extend(removed.clone());
            removed.clear();

            let curr_len: usize = p2_map_without_b1
                .iter()
                .map(|(_, v)| v.len())
                .sum();
            if curr_len != prev_len {
                something_changed = true;
            }

            for (key, val) in p2_map_without_b1.clone() {
                if val.len() == 0 && prev_map[&key].len() != 0{
                    removed.insert(key);
                    // print_brick(key);
                }
            }
        }

        // //DEBUG
        // println!("Removed: {}", total_removed.len() - 1); // It would count itself too
        // if total_removed.len() > 1 {
        //     print_brick(*b1);
        //     println!("{:?}", total_removed);
        // }
        // println!("Remaining: {}", p2_map_without_b1.len());
        p2_sum += total_removed.len() - 1;
    }
    println!("Part 2: {}", p2_sum); // 1305 is too low
}

// Helper function that returns the bricks if they all straight down.
fn give_fallen_bricks(v: &Vec<Brick>) -> Vec<Brick> {
    // The floor is at z = 0, so when a blocks z1 or z2 is 1 it lies on the floor.
    let mut v = v.clone();

    // Repeatedly try letting a brick fall. If it overlaps with another brick, try again.
    let mut somethgin_changed = true;
    while somethgin_changed {
        somethgin_changed = false;
        for i in 0..v.len() {
            let mut brick = v[i];
            if brick.0 .2 <= 1 || brick.1 .2 <= 1 {
                continue;
            }
            // brick .0 .2 -= 1;
            // brick .1 .2 -= 1;

            // let overlap = v
            //     .iter()
            //     .enumerate()
            //     .filter(|(j, _)| i != *j)
            //     .any(|(_, b)| bricks_overlap(brick, *b));

            // if overlap {
            //     brick .0 .2 += 1;
            //     brick .1 .2 += 1;
            // } else {
            //     somethgin_changed = true;
            //     print_brick(brick);
            // }

            let z_dist = v
                .iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .filter_map(|(_, b)| zdistance2(*b, brick).map(|x| x - 1))
                .filter(|&x| x >= 0)
                .min();

            if z_dist.is_some() {
                // println!("Possible move: {}", z_dist.unwrap());
                // move it down

                // DEBUG
                // if brick.1.0 == 0 {
                //     print_brick(brick);
                //     for b in &v {
                //         print_brick(*b);
                //         println!("Distance: {:?}", zdistance(brick, *b));
                //     }
                // }
                if z_dist.unwrap() > 0 {
                    somethgin_changed = true;
                    brick.0 .2 -= z_dist.unwrap();
                    brick.1 .2 -= z_dist.unwrap();
                    // print_brick(brick);
                    // print!("{} ", z_dist.unwrap());
                    v[i] = brick;
                }
            }
        }
    }

    v
}

// Helper function that returns the bricks if they all straight down.
fn give_fallen_bricks_old(v: &Vec<Brick>) -> Vec<Brick> {
    // The floor is at z = 0, so when a blocks z1 or z2 is 1 it lies on the floor.
    let mut v = v.clone();

    // Repeatedly try letting a brick fall. If it overlaps with another brick, try again.
    let mut somethgin_changed = true;
    while somethgin_changed {
        somethgin_changed = false;
        for i in 0..v.len() {
            let mut brick = v[i];
            if brick.0 .2 <= 1 || brick.1 .2 <= 1 {
                continue;
            }
            brick.0 .2 -= 1;
            brick.1 .2 -= 1;

            let overlap = v
                .iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .any(|(_, b)| bricks_overlap(brick, *b));

            if overlap {
                brick.0 .2 += 1;
                brick.1 .2 += 1;
            } else {
                somethgin_changed = true;
                // print_brick(brick);
            }
            v[i] = brick;
        }
    }

    v
}

// Helper function that returns true if two bricks overlap
fn bricks_overlap(b1: Brick, b2: Brick) -> bool {
    return overlap((b1.0 .0, b1.1 .0), (b2.0 .0, b2.1 .0)).is_some()
        && overlap((b1.0 .1, b1.1 .1), (b2.0 .1, b2.1 .1)).is_some()
        && overlap((b1.0 .2, b1.1 .2), (b2.0 .2, b2.1 .2)).is_some();
}

// Helper function that return NONE if two bricks don't intersect  in the xy plane
// and else the z distance between them
fn zdistance(b1: Brick, b2: Brick) -> Option<i32> {
    let x = overlap((b1.0 .0, b1.1 .0), (b2.0 .0, b2.1 .0));
    let y = overlap((b1.0 .1, b1.1 .1), (b2.0 .1, b2.1 .1));
    if x.is_none() || y.is_none() {
        return None;
    }
    let z = overlap((b1.0 .2, b1.1 .2), (b2.0 .2, b2.1 .2));
    if z.is_some() {
        return None;
    }
    let b1z_min = b1.0 .2.min(b1.1 .2);
    let b1z_max = b1.0 .2.max(b1.1 .2);
    let b2z_min = b2.0 .2.min(b2.1 .2);
    let b2z_max = b2.0 .2.max(b2.1 .2);

    let z_min = b1z_min.max(b2z_min);
    let z_max = b1z_max.min(b2z_max);

    let z_distance = z_min - z_max;
    return Some(z_distance); // signed
}

// Helper function that return the zdistance between two bricks unless b2 is above b1, then NONE
fn zdistance2(b1: Brick, b2: Brick) -> Option<i32> {
    let zdist = zdistance(b1, b2);
    if zdist.is_none() {
        return None;
    }
    let b1z_min = b1.0 .2.min(b1.1 .2);
    let b2z_max = b2.0 .2.max(b2.1 .2);

    if b1z_min > b2z_max {
        return None;
    }
    zdist
}

// Helper function that returns overlap given two intervals
fn overlap(a: (i32, i32), b: (i32, i32)) -> Option<(i32, i32)> {
    let (a1, a2) = a;
    let (b1, b2) = b;
    if a1 <= b1 && b1 <= a2 {
        Some((b1, a2))
    } else if a1 <= b2 && b2 <= a2 {
        Some((a1, b2))
    } else if b1 <= a1 && a1 <= b2 {
        Some((a1, b2))
    } else if b1 <= a2 && a2 <= b2 {
        Some((b1, a2))
    } else {
        None
    }
}

// Helper function that prints a brick
fn print_brick(brick: Brick) {
    let ((x1, y1, z1), (x2, y2, z2)) = brick;
    println!(
        "Brick from ({}, {}, {}) to ({}, {}, {})",
        x1, y1, z1, x2, y2, z2
    );
}
