use std::collections::HashMap;

fn main() {
    let text = include_str!("TEMP.txt");
    // let text = include_str!("TEMP2.txt");
    // let text = include_str!("TEMP3.txt");

    let elements = text.split(",").collect::<Vec<&str>>();

    // Part 2
    //
    let mut hashmap: HashMap<u8, Vec<(String, i32)>> = HashMap::new();

    // For each character, take ascii value, times 17 mod 256 and add new character
    let elements2 = elements
        .iter()
        .map(|x| {
            let mut sum = 0;
            for c in x.chars() {
                sum += (c as u8) as u32;
                sum *= 17;
                sum %= 256;
            }

            match (x.find("="), x.find("-")) {
                (Some(i), None) => {
                    // Find and replace the element in the array
                    let key = x[..i].to_string();
                    let value = x[i + 1..].parse::<i32>().unwrap();

                    let mut key_sum = 0;
                    for c in key.chars() {
                        key_sum += (c as u8) as u32;
                        key_sum *= 17;
                        key_sum %= 256;
                    }
                    // if the inner vector contains the key, replace it
                    let inner_index = hashmap
                        .entry(key_sum as u8)
                        .or_insert(Vec::new())
                        .iter()
                        .position(|x| x.0 == key)
                        .or(None);
                    
                    match inner_index {
                        None => {
                            hashmap
                                .entry(key_sum as u8)
                                .or_insert(Vec::new())
                                .push((key, value));
                        }
                        Some(i) => {
                            hashmap
                                .entry(key_sum as u8)
                                .or_insert(Vec::new())
                                .get_mut(i)
                                .unwrap()
                                .1 = value;
                        }
                    }
                }
                (None, Some(i)) => {
                    // Remove the element from the array
                    let key = x[..i].to_string();

                    let mut key_sum = 0;
                    for c in key.chars() {
                        key_sum += (c as u8) as u32;
                        key_sum *= 17;
                        key_sum %= 256;
                    }

                    let inner_index = hashmap
                        .entry(key_sum as u8)
                        .or_insert(Vec::new())
                        .iter()
                        .position(|x| x.0 == key)
                        .or(None);
                    if let Some(inner_index) = inner_index {
                        hashmap
                            .entry(key_sum as u8)
                            .or_insert(Vec::new())
                            .remove(inner_index);
                    }
                }
                _ => {
                    panic!("Invalid input: {}", x);
                }
            }

            sum
        })
        .collect::<Vec<u32>>();

    println!("sum: {}", elements2.iter().sum::<u32>());
    // println!("elements: {:?}", elements);

    println!("hashmap: {:?}", hashmap);

    // Part2 calculation
    let mut sum = 0;
    for (key, value) in hashmap.iter() {
        for (i, (_, value2)) in value.iter().enumerate() {
            let plus = (*key as i32 + 1) * value2 * (i as i32 + 1);
            // dbg!(plus, key, value2, i);
            sum += plus;
        }
    }

    println!("sum: {}", sum);
}
