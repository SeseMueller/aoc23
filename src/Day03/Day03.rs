use std::collections::HashMap;

fn main() {
    
   let text = include_str!("TEMP.txt");
//    let text = include_str!("TEMP2.txt");

    // COnvert to 2D array
    let grid = text.lines()
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Go over grid and replace every number that has no symbol adjacent to it with a period
    let mut grid2 = Vec::new();

    for (idx,line) in grid.iter().enumerate() {

        let mut line2 = Vec::new();

        for (idx2, c) in line.iter().enumerate() {

            if c.is_numeric() {
                let mut has_symbol = false;
                for i in -1i32..2 {
                    for j in -1i32..2 {
                        let ind1 = idx as i32 +i;
                        let ind2 = idx2 as i32 +j;
                        if ind1 >= grid.len() as i32|| ind2 >= grid[0].len() as i32{
                            continue;
                        }
                        if ind1 < 0 || ind2 < 0 {
                            continue;
                        }
                        let entry = grid [ind1 as usize][ind2 as usize];
                        if entry != '.' && !entry.is_numeric() {
                            has_symbol = true;
                        }
                    }
                }
                if has_symbol {
                    line2.push(*c);
                } else {
                    line2.push('.');
                }
            } else {
                line2.push(*c);
            }
        }
        grid2.push(line2);
    }

    // Go over grid1 and find all numbers 
    // Where "025" mean 25 and not 2, 5.

    let mut numbers = Vec::new();

    for (idx,line) in grid.iter().enumerate() {
        
        let mut buffer = Vec::new();
        for (idx2, c) in line.iter().enumerate() {
            if c.is_numeric() {
                buffer.push((*c, idx, idx2));
            } else {
                if buffer.len() > 0  {
                    let mut num = String::new();
                    for (c,_,_) in buffer.iter() {
                        num.push(*c);
                    }
                    numbers.push((num, buffer[0].1, buffer[0].2));
                    buffer = Vec::new();
                }
            }
        }

        if buffer.len() > 0  {
            let mut num = String::new();
            for (c,_,_) in buffer.iter() {
                num.push(*c);
            }
            numbers.push((num, buffer[0].1, buffer[0].2));
            // buffer = Vec::new();
        }
                
    }

    // Go over numbers and remove all numbers that have been replaced with a period everywhere

    let mut numbers2 = Vec::new();

    for (num, idx, idx2) in numbers {
        let mut num_periods = 0;
        for i in (idx2)..(idx2+num.len()) {
            if grid2[idx][i] == '.' {
                num_periods += 1;
            }
        }
        if num_periods != num.len() {
            numbers2.push((num, idx, idx2));
        }
    }

    let numbers3 = numbers2.iter().map(|(num,_,_)| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    // println!("numbers3: {:?}", numbers3);
    println!("numbers3: {:?}", numbers3.iter().sum::<i32>());

    // Part 2: Find (*) and it's adjacent numbers and multiply them together

    let mut gear_pos = Vec::new();
    for (idx,line) in grid.iter().enumerate() {
        for (idx2, c) in line.iter().enumerate() {
            if *c == '*' {
                gear_pos.push((idx, idx2));
            }
        }
    }

    // Use numbers2 to find the numbers that are adjacent to the gear_pos

    let mut gear_numbers = HashMap::new();

    for (idx, idx2) in gear_pos {
        gear_numbers.insert((idx, idx2), Vec::new());
        for (num, idx3, idx4) in numbers2.iter() {
            // The numbers can be +1 off horizontally and STRING.len() off vertically
            if *idx3 as i32-1 <= idx as i32 && idx2 <= idx4+num.len() && *idx4 as i32-1 <= idx2 as i32 && idx <= idx3+1 {
                // gear_numbers.push(num.parse::<i32>().unwrap());
                // gear_numbers.insert((idx, idx2), num.parse::<i32>().unwrap());
                gear_numbers.get_mut(&(idx, idx2)).unwrap().push(num.parse::<i32>().unwrap());
            }
        }
    }

    // println!("gear_numbers: {:?}", gear_numbers);

    let gear_number:i32 = gear_numbers.iter()
    .filter(|(_,v)| v.len() == 2)
    .map(|(_,v)| v.iter().product::<i32>())
    .sum();

    println!("gear_number: {:?}", gear_number);
}
