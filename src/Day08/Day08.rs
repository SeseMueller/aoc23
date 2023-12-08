use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let text = include_str!("TEMP.txt");
    // let text = include_str!("TEMP2.txt");

    let (leftright, nodestext) = text
        .split("\n\n")
        .collect_tuple()
        .expect("Error splitting text");

    let mut nodes = HashMap::new();

    for line in nodestext.lines() {
        let (name, dests) = line.split(" = ").collect_tuple().unwrap();
        // dbg!("{} -> {}", name, dests);
        let dest = dests.strip_prefix("(").unwrap().strip_suffix(")").unwrap();
        let (left, right) = dest.split(", ").collect_tuple().unwrap();

        nodes.insert(name, (left, right));
    }

    // // Start at AAA, follow the leftright instructions until we hit ZZZ

    // let mut steps = 0;
    // let mut i = 0;
    // let mut current = "AAA";

    // // loop forever over the leftright instructions
    // while current != "ZZZ" {
    //     let (left, right) = nodes.get(current).unwrap();

    //     if leftright.chars().nth(i).unwrap() == 'L' {
    //         current = left;
    //     } else {
    //         current = right;
    //     }

    //     i += 1;
    //     i %= leftright.len();
    //     steps += 1;
    // }

    // println!("steps: {}", steps);

    // Part 2: simulate all nodes ending with A until they are on a node ending with Z

    // let mut mapped_nodes = nodes.clone();
    // mapped_nodes.retain(|k, _| k.ends_with("A"));

    // let mut steps = 0;
    // let mut i = 0;

    // while mapped_nodes.keys().any(|k| !k.ends_with("Z")) {
    //     let mut new_nodes = HashMap::new();

    //     let lr = leftright.chars().nth(i).unwrap();

    //     for (_, (left, right)) in mapped_nodes.iter() {
    //         if lr == 'L' {
    //             new_nodes.insert(left.to_owned(), nodes.get(left).unwrap().clone());
    //         } else {
    //             new_nodes.insert(right, nodes.get(right).unwrap().clone());
    //         }
    //     }

    //     mapped_nodes = new_nodes;

    //     i += 1;
    //     i %= leftright.len();
    //     steps += 1;

    //     // DEbug
    //     if steps % 1000000 == 0 {
    //         println!("steps: {}", steps);
    //     }
    // }

    // println!("steps: {}", steps);

    // Attempt 2: find the looping number and offset for each node

    // (node, offset, looplength, loopnode)
    let mut loopings: HashMap<String, (String, i32, i32, String)> = HashMap::new();

    // This takes a while, use -r to run it
    for node in nodes.keys() {
        let mut i = 0;
        let mut current = node;

        let mut seen: HashSet<(usize, String)> = HashSet::new(); // The i is also important

        // loop forever over the leftright instructions
        while !seen.contains(&(i, current.to_string())) {
            seen.insert((i, current.to_string()));

            let (left, right) = nodes.get(current).unwrap();

            if leftright.chars().nth(i).unwrap() == 'L' {
                current = left;
            } else {
                current = right;
            }

            i += 1;
            i %= leftright.len();
        }

        // println!("steps: {}", steps);

        // Now, we know that the loop starts at i, and at the current node

        let mut looplength = 0;
        let loopnode = current;

        let mut current = node;
        let mut tempi = 0;

        loop {
            let (left, right) = nodes.get(current).unwrap();

            if leftright.chars().nth(tempi).unwrap() == 'L' {
                current = left;
            } else {
                current = right;
            }

            tempi += 1;
            tempi %= leftright.len();
            looplength += 1;

            if current == loopnode && tempi == i {
                break;
            }
        }

        // println!("looplength: {}", looplength);

        loopings.insert(
            node.to_string(),
            (
                loopnode.to_string(),
                i as i32,
                looplength as i32,
                loopnode.to_string(),
            ),
        );
    }

    // println!("looping: {:?}", loopings);

    let mut num:i128 = 1;

    // for (_, (_, off, looplength, _)) in loopings.iter() {
    for (_, (_, _, looplength, _)) in loopings.iter().filter(|(k, _)| k.ends_with("Z")) {
        num = lcm(num, looplength.clone() as i128);
        println!("lcm: {}", num);
    }
}

fn lcm(a: i128, b: i128) -> i128 {
    a * b / gcd(a, b)
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}
