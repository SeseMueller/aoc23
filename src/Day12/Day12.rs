use std::time::Duration;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Field {
    Empty,
    Filled,
    Unknown,
    Occupied,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Memo {
    map: Vec<Field>,
    nums: Vec<usize>,
}

fn main() {

    let text = include_str!("TEMP.txt");
    // let text = include_str!("TEMP2.txt");
    // let text = include_str!("TEMP3.txt");


    // Today, we generate the numer of possible nonogram solutions for a given row.

    let mut sum1 = 0;
    let mut sum2 = 0;

    for line in text.lines() {
        let (row, nums) = line.split_whitespace().collect_tuple().unwrap();

        let row:Vec<Field> = row.chars().map(|c| {
            match c {
                '.' => Field::Empty,
                '#' => Field::Filled,
                '?' => Field::Unknown,
                _ => panic!("Unknown character: {}", c),
            }}).collect();

        let nums:Vec<usize> = nums.split(',').map(|s| s.parse().unwrap()).collect();

        // Dynamic programming.

        let solutions = solve_row2(row.clone(), nums.clone());

        // dbg!(solutions);
        sum1 += solutions;

        // Part 2: quintuple the row (copy 4 time, place ? in between), and quintuple the numbers (copy 4 times)
        let mut temp_row = row; 
        temp_row.push(Field::Unknown);
        let mut new_row:Vec<Field> = temp_row.repeat(5);
        new_row.pop();

        let new_nums = nums.repeat(5);


        // dbg!(new_row.clone(), new_nums.clone());

        let solutions = solve_row2(new_row, new_nums);

        dbg!(solutions);

        sum2 += solutions;

    }

    println!("Sum: {}", sum1);
    println!("Sum: {}", sum2);

    // // DEBUG:
    // let new_row:Vec<Field> = vec![Field::Unknown, Field::Unknown, Field::Occupied];
    // let new_nums:Vec<usize> = vec![1];

    // dbg!(solve_row2(&new_row, &new_nums));

    // let new_row2:Vec<Field> =  new_row.iter().flat_map(|&f| {
    //     vec![f, Field::Unknown, f, Field::Unknown, f, Field::Unknown, f, Field::Unknown, f]
    // }).collect();

    // let new_nums2:Vec<usize> = new_nums.iter().flat_map(|&n| {
    //     vec![n, n, n, n, n]
    // }).collect();

    // dbg!(solve_row2(&new_row2, &new_nums2));



}

fn solve_row(row: &Vec<Field>, nums: &Vec<usize>, ognums:&Vec<usize>, minindex: usize) -> i32 {

    // dbg!(row, nums);

    if nums.is_empty() {
        // make sure that each field is either unkown, occupied or empty.
        if row.iter().all(|&f| f == Field::Unknown || f == Field::Occupied || f == Field::Empty) {
            return 1;
        } else {
            return 0;
        }
    }

    // Also check that there are not more filled than remaining numbers.
    if row[minindex..].iter().filter(|&&f| f == Field::Filled).count() > nums.iter().sum::<usize>() {
        return 0;
    }
    
    let mut possible_indecies = Vec::new();

    for (i, w) in row.windows(nums[0]).enumerate() {
        if w.iter().all(|&f| f == Field::Filled || f == Field::Unknown) {
            possible_indecies.push(i);
        }
    }

    // dbg!(possible_indecies.clone());

    // This number needs to be after the last number, so after minindex.
    possible_indecies = possible_indecies.iter().filter(|&&i| i >= minindex).map(|&i| i).collect();

    // The location of the first number cannot be such that the field right before or after it is filled.

    possible_indecies = possible_indecies.iter().filter(|&&i| {
        if i == 0 {
            row[i + nums[0]] != Field::Filled && row[i + nums[0]] != Field::Occupied
        } else if i + nums[0] == row.len() {
            row[i - 1] != Field::Filled && row[i - 1] != Field::Occupied
        } else {
            row[i - 1] != Field::Filled && row[i + nums[0]] != Field::Filled && row[i - 1] != Field::Occupied && row[i + nums[0]] != Field::Occupied
        }
    }).map(|&i| i).collect();

    let mut sum = 0;

    // For each possible index, we construct a new row, and call solve_row on that row and the remaining numbers.
    for index in possible_indecies {
        let mut new_row = row.clone();
        for i in index..index + nums[0] {
            new_row[i] = Field::Occupied;
        }

        let new_nums = nums.iter().skip(1).map(|&n| n).collect();

        let solutions = solve_row(&new_row, &new_nums, ognums, index + nums[0]);

        sum += solutions;

        // dbg!(solutions, sum);
    }

    sum
}

#[memoize::memoize(Capacity: 10000, TimeToLive: Duration::from_secs(30))] // Thanks a lot, this is so much faster!
fn solve_row2(row: Vec<Field>, nums: Vec<usize>) -> i64 {

    // Because we split the row into 2 here, find a middle index.
    let middle_index = nums.len() / 2;
    // dbg!(middle_index, nums);
    
    
    if row.is_empty() {
        return 0;
    }
    
    if nums.is_empty() {
        // make sure that each field is either unkown, occupied or empty.
        if row.iter().all(|&f| f == Field::Unknown || f == Field::Occupied || f == Field::Empty) {
            return 1;
        } else {
            return 0;
        }
    }
    
    let mut possible_indecies = Vec::new();

    // dbg!(row, nums);

    for (i, w) in row.windows(nums[middle_index]).enumerate() {
        if w.iter().all(|&f| f == Field::Filled || f == Field::Unknown) {
            possible_indecies.push(i);
        }
    }

    if possible_indecies.is_empty() {
        return 0;
    }

    // dbg!(possible_indecies.clone());
    
    // The location of the first number cannot be such that the field right before or after it is filled.
    
    possible_indecies = possible_indecies.iter().filter(|&&i| {
        let a = if i + nums[middle_index] < row.len(){
            row[i + nums[middle_index]] != Field::Filled && row[i + nums[middle_index]] != Field::Occupied
        } else { true };
        let b = if i > 0 {
            row[i - 1] != Field::Filled && row[i - 1] != Field::Occupied
        } else { true };
        a && b
    }).map(|&i| i).collect();
    
    // dbg!(possible_indecies.clone(), middle_index, nums, row);

    if possible_indecies.is_empty() {
        return 0;
    }

    // dbg!(row, nums);

    let mut sum = 0;
    
    // For each possible index, we construct a new row, and call solve_row on that row and the remaining numbers.
    // ACtually, dont. It's too slow.
    // Because of the strong ordering of the numbers, we can just leave out 
    for index in possible_indecies {
        let mut new_row = row.clone();
        for i in index..index + nums[middle_index] {
            new_row[i] = Field::Occupied;
        }

        // Bifurcate the row into 2.
        let (left_row, right_row) = (new_row[..index + nums[middle_index]].to_vec(), new_row[index..].to_vec());

        // dbg!(nums, middle_index);
        // Bifurcate the numbers into 2.
        let (left_nums, right_nums) = (nums[..middle_index].to_vec(), nums[middle_index + 1..].to_vec());
        // Also remove the last number from the left nums, because it is already placed.
        // let left_nums = left_nums.iter().take(left_nums.len() - 1).map(|&n| n).collect();

        let left_solutions = solve_row2(left_row.to_vec(), left_nums);

        if left_solutions == 0 {
            continue; // No need to check the right side.
        }
        let right_solutions = solve_row2(right_row.to_vec(), right_nums.to_vec());



        sum += left_solutions * right_solutions; // Also nicely works for 0 solutions.

        // if left_solutions > 0 && ! left_nums.is_empty(){
        //     dbg!(left_row, left_nums, left_solutions);
        // }
        // if right_solutions > 0 && !right_nums.is_empty() {
        //     dbg!(right_row, right_nums, right_solutions);
        // }
    }

    sum
}
// Possible later optimizations: dynamic programming instead of just memoization. 10s is good enough for now though.