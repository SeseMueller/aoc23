fn test() {
    let text = include_str!("Day01.txt");
    let test:i32 = text.lines()
        .into_iter()
        // Only keep digits
        .map(|line| line.chars()
            .into_iter()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
        )
        .map(|line|
        match line.len() {
            1 => line.clone() + line.as_str(),
            2.. => {
                // Get first and last char
                let first = line.chars().nth(0).unwrap();
                let last = line.chars().nth(line.len() - 1).unwrap();
                first.to_string() + last.to_string().as_str()
            }
            _ => panic!("Unexpected length in line: {}", line),}
        )
        .map(|line| line.parse::<i32>().unwrap())
        .sum();
    println!( "{:?}", test);

        // Part 2

        // Same, but replace "one" with 1, etc. to "nine" with 9


        let test2:i32 = text.lines()
        .into_iter()
        .map(|line|
            line.replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
            
        )
        // Only keep digits
        .map(|line| line.chars()
            .into_iter()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
        )
        .map(|line|
        match line.len() {
            1 => line.clone() + line.as_str(),
            2.. => {
                // Get first and last char
                let first = line.chars().nth(0).unwrap();
                let last = line.chars().nth(line.len() - 1).unwrap();
                first.to_string() + last.to_string().as_str()
            }
            _ => panic!("Unexpected length"),}
        )
        .map(|line| line.parse::<i32>().unwrap())
        .sum();
    println!( "{:?}", test2);

}