use std::{
    cmp::{max, min},
    collections::HashSet,
};

fn main() {
    let text = include_str!("TEMP.txt");
    //  let text = include_str!("TEMP2.txt");

    let parts = text.split("\n\n").collect::<Vec<_>>();

    // remove any non-numeric characters from the front of the string
    let parts = parts
        .iter()
        .map(|x| x.trim_start_matches(|a: char| !a.is_numeric()).trim())
        .collect::<Vec<_>>();

    // println!("parts: {:?}", parts);

    let bases = parts[0]
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    println!("bases: {:?}", bases);

    // Each line in the second to last part is a 3-list of numbers
    // where the first number is the start of the destination range
    // the second number is the start of the source range
    // the third number is the length of the range

    // So it's a mapping problem.
    // First, parse everything into a list of list of tuples.

    let mut mappings = Vec::new();

    for part in parts[1..].iter() {
        // Split into lines
        let lines = part.split("\n").collect::<Vec<_>>();
        let mut maps = Vec::new();

        for line in lines {
            // Split into numbers
            let nums = line
                .split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            // Split into tuple
            if nums.len() != 3 {
                panic!("Invalid line length: {}", line);
            }
            let tuple = (nums[0], nums[1], nums[2]);
            maps.push(tuple);
        }
        mappings.push(maps);
    }

    // println!("mappings: {:?}", mappings);

    // Now, maps the bases to the final values.

    let mut final_values = Vec::new();

    for base in bases.clone() {
        let mut value = base;
        for map in mappings.iter() {
            for (dest, src, len) in map.iter() {
                if value >= *src && value < *src + *len {
                    value = *dest + (value - *src);
                    break;
                }
            }
        }
        final_values.push(value);
    }

    //  println!("final_values: {:?}", final_values);

    // min
    println!("min: {}", final_values.iter().min().unwrap());

    // Part 2:
    // Transform bases into a range of numbers

    let mut ranges = Vec::new();

    for base in bases.chunks(2) {
        let range = base[0]..(base[1] + base[0]);
        ranges.push(range);
    }

    //  println!("ranges: {:?}", ranges);

    // Map them again, splitting them if they overlap only partially

    let mut temp_ranges: HashSet<std::ops::Range<i64>> = ranges.iter().cloned().collect();

    for maps in mappings {
        let mut new_ranges: HashSet<std::ops::Range<i64>> = temp_ranges.iter().cloned().collect();
        let mut new_mapped_ranges: HashSet<std::ops::Range<i64>> = HashSet::new();

        while let Some(range) = {
            if new_ranges.len() == 0 {
                None
            } else {
                let range = new_ranges.iter().next().unwrap().clone();
                new_ranges.remove(&range);
                Some(range)
            }
        } {
            // dbg!(new_ranges.len(), new_mapped_ranges.len());
            // dbg!(new_ranges.clone());
            let mut something_changed = false;
            for (dest, src, len) in maps.iter() {
                // Find all ranges that overlaps with src

                //  dbg!(range.start, range.end, *src, *src + *len);
                if (range.start <= (*src + *len)) && (range.end >= *src) {
                    something_changed = true;
                    // Split range into potentially 3 ranges

                    // Before src
                    if range.contains(src) {
                        // dbg!(range.start, range.end, *src, *len);
                        let new_range = range.start..*src;
                        if new_range.end > new_range.start {
                            new_ranges.insert(new_range);
                        }
                    }

                    // After src
                    if range.contains(&(src + len - 1)) {
                        // -1?????
                        // dbg!(range.start, range.end, *src, *len);
                        let new_range = *src + *len..range.end;
                        if new_range.end > new_range.start {
                            new_ranges.insert(new_range);
                        }
                    }

                    // The range that is being mapped
                    let mapped_start = max(range.start, *src);
                    let mapped_end = min(range.end, *src + *len);
                    let new_range = *dest + (mapped_start - *src)..*dest + (mapped_end - *src);
                    new_mapped_ranges.insert(new_range);
                }
            }

            if !something_changed {
                new_mapped_ranges.insert(range.clone());
            }
        }

        temp_ranges = new_mapped_ranges;
    }

    //  println!("temp_ranges: {:?}", temp_ranges);

    // Min

    println!(
        "min: {}",
        temp_ranges
            .iter()
            .map(|x| x.start)
            .min()
            .unwrap_or(std::i64::MAX)
    );
}
