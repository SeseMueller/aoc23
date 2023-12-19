use std::{collections::HashMap, ops::Range};

use itertools::Itertools;

#[derive(Debug)]
enum Mapping {
    LessThan(u16, String),
    GreaterThan(u16, String),
    Reject,
    Accept,
    Redirect(String),
}

#[derive(Debug)]
struct Workflow {
    mappings: Vec<(char, Mapping)>,
    else_do: String,
}

fn main() {
    let text = include_str!("TEMP.txt");
    // let text = include_str!("TEMP2.txt");
    // let text = include_str!("TEMP3.txt");

    // Read input mapppings
    let (mappings, vars) = text.split("\n\n").collect_tuple().unwrap();

    let mappings = mappings
        .lines()
        .map(|line| {
            let (key, ms) = line.split('{').collect_tuple().unwrap();
            let ms = ms.trim_end_matches('}');
            // Parse mappings
            let ms = ms.split(',').collect_vec();

            let else_do = ms.clone().last().unwrap().to_string();

            let ms = ms[..ms.len() - 1]
                .iter()
                .map(|m| {
                    // dbg!(m);
                    let v = m.chars().next().unwrap(); // Variable that is being mapped
                    let op = m.chars().nth(1).unwrap(); // Operator
                    let (num, send_to) = m[2..].split(':').collect_tuple().unwrap(); // Operands

                    // dbg!(v, op, num, send_to);

                    let m = match op {
                        '<' => Mapping::LessThan(num.parse::<_>().unwrap(), send_to.to_string()),
                        '>' => Mapping::GreaterThan(num.parse::<_>().unwrap(), send_to.to_string()),
                        ',' => {
                            if send_to == "R" {
                                Mapping::Reject
                            } else if send_to == "A" {
                                Mapping::Accept
                            } else {
                                Mapping::Redirect(send_to.to_string())
                            }
                        }
                        e => panic!("Invalid operator \"{}\" in string: {}", e, m),
                    };

                    (v, m)
                })
                .collect_vec();

            (
                key,
                Workflow {
                    mappings: ms,
                    else_do,
                },
            )
        })
        .collect_vec();

    // dbg!(mappings);

    let mut part1: u64 = 0;

    // For each line, read variables and apply mappings
    for line in vars.lines() {
        // Line is a string of variables in the form "{A=1,B=2,C=3}"
        let line = line.trim_start_matches('{').trim_end_matches('}');

        // dbg!(line);

        // Parse variables
        let vars = line.split(',').collect_vec();

        // dbg!(vars);

        let vars = vars
            .iter()
            .map(|v| {
                let (var, val) = v.split('=').collect_tuple().unwrap();
                (var.chars().next().unwrap(), val.parse::<u16>().unwrap())
            })
            .collect::<HashMap<_, _>>();

        // dbg!(vars);
        //Apply mappings
        let accept = workflows_accept_variables(&mappings, &vars, "in".to_string());
        // dbg!(accept);

        if accept {
            // In first part, add up all the variables
            let sum = vars.values().sum::<u16>();
            // dbg!(sum);
            part1 += sum as u64;
        }
    }

    println!("Part 1: {}", part1);

    // Part 2:
    // Instead of using the supplied variable, find how many variable combinations (with all four ranging from 1 to 4000) will be accepted
    // To do that, initialize them to 1..=4000 and then apply the mappings

    let mut vars = HashMap::new();
    vars.insert('x', 1..4001);
    vars.insert('m', 1..4001);
    vars.insert('a', 1..4001);
    vars.insert('s', 1..4001);

    let combinations = get_num_accept_combinations(&mappings, &vars, "in".to_string());

    println!("Part 2: {}", combinations);
}

fn workflows_accept_variables(
    mappings: &Vec<(&str, Workflow)>,
    vars: &HashMap<char, u16>,
    apply_name: String,
) -> bool {
    // Apply Workflow apply_name to variables vars
    // If it rejects, return false; if it accepts, return true; if it redirects, apply the redirect and return the result of that workflow
    // dbg!(apply_name.clone());

    if apply_name == "A" {
        return true;
    }; // Else do might accept or reject
    if apply_name == "R" {
        return false;
    };
    let w = &mappings.iter().find(|w| w.0 == apply_name).unwrap().1;

    for (var, mapping) in w.mappings.iter() {
        let var = *var;
        let val = vars.get(&var).unwrap();

        match mapping {
            Mapping::LessThan(num, send_to) => {
                if val < num {
                    return workflows_accept_variables(mappings, vars, send_to.to_string());
                }
            }
            Mapping::GreaterThan(num, send_to) => {
                if val > num {
                    return workflows_accept_variables(mappings, vars, send_to.to_string());
                }
            }
            Mapping::Reject => return false,
            Mapping::Accept => return true,
            Mapping::Redirect(send_to) => {
                return workflows_accept_variables(mappings, vars, send_to.to_string())
            }
        }
    }
    // Else do
    workflows_accept_variables(mappings, vars, w.else_do.to_string())
}

fn get_num_accept_combinations(
    mappings: &Vec<(&str, Workflow)>,
    in_vars: &HashMap<char, Range<u16>>,
    apply_name: String,
) -> u128 {
    // Apply Workflow apply_name to variables vars
    // When it is applied, tighten the range of the variables to the range that is or used.
    if apply_name == "A" {
        return get_num_combinations(in_vars);
    }; // Else do might accept or reject
    if apply_name == "R" {
        return 0;
    };

    let mut sum = 0;

    let vars: &mut HashMap<char, Range<u16>> = &mut in_vars.clone();

    for (var, mapping) in mappings
        .iter()
        .find(|w| w.0 == apply_name)
        .unwrap()
        .1
        .mappings
        .iter()
    {
        // dbg!(var, mapping);

        let var = *var;
        let val = vars.get(&var).unwrap();

        match mapping {
            Mapping::LessThan(num, send_to) => {
                if val.start < *num {
                    // dbg!("Before:", vars.clone(), sum);
                    let mut new_vars = vars.clone();
                    assert!(new_vars.insert(var, val.start..*num).is_some());
                    // assert!(vars.insert(var, val.start..*num).is_some());

                    sum += get_num_accept_combinations(mappings, &new_vars, send_to.to_string());
                    // Also restrict the other ranges by the opposite
                    assert!(vars.insert(var, *num..val.end).is_some());
                    // dbg!("After:", vars.clone(), sum);
                }
            }
            Mapping::GreaterThan(num, send_to) => {
                if val.end > *num {
                    // dbg!("Before:", vars.clone(), sum);
                    let mut new_vars = vars.clone();
                    assert!(new_vars.insert(var, *num+1..val.end).is_some()); // This +1 took me like half an hour to find
                    // assert!(vars.insert(var, *num..val.end).is_some());
                    sum += get_num_accept_combinations(mappings, &new_vars, send_to.to_string());
                    // Also restrict the other ranges by the opposite
                    assert!(vars.insert(var, val.start..*num+1).is_some());
                    // dbg!("After:", vars.clone(), sum);
                }
            }
            Mapping::Reject => sum += 0,
            Mapping::Accept => sum += get_num_combinations(vars),
            Mapping::Redirect(_) => {
                // sum += get_num_accept_combinations(mappings, vars, send_to.to_string())
            }
        }
    }

    // Else do
    sum += get_num_accept_combinations(
        mappings,
        vars,
        mappings.iter().find(|w| w.0 == apply_name).unwrap().1.else_do.to_string(),
    );

    sum
}

fn get_num_combinations(vars: &HashMap<char, Range<u16>>) -> u128 {
    // Get the number of combinations of variables
    // For each variable, get the number of values it can take and multiply them all together
    vars.iter().map(|(_, v)| v.len() as u128).product()
}

// But it schould be   167409079868000
// Ok, so I get        56597805000000 
// Ok, now I'm getting 640834366672426...
// getting closer      167076220640454
// even closer         167474394229030