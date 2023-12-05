fn main() {
    let text = include_str!("Day04.txt");

    let mut num_wins = Vec::new();
    let mut num_wins_raw = Vec::new();

    for line in text.lines() {
        // Remove the "Card X: " prefix
        let line = line.trim_start_matches("Card ");
        let line = line.trim();
        let line = line.trim_start_matches(char::is_numeric);
        let line = line.trim_start_matches(char::is_numeric);
        let line = line.trim_start_matches(char::is_numeric);
        let line = line.trim_start_matches(":");

        let parts = line.split("|").collect::<Vec<_>>();
        if parts.len() != 2 {
            println!("Invalid line: {}", line);
            continue;
        }
        let part1 = parts[0].trim();
        let part2 = parts[1].trim();

        // dbg!(part1);
        // dbg!(part2);

        let part1 = part1
            .split(" ")
            .filter(|x| x != &"")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let part2 = part2
            .split(" ")
            .filter(|x| x != &"")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        // FInd overlap
        let mut overlap = Vec::new();
        for i in &part1 {
            for j in &part2 {
                if i == j {
                    overlap.push(*i);
                }
            }
        }

        // 2 ^ (overlap.len() - 1); no points for -1
        num_wins.push(if overlap.len() == 0 {
            0
        } else {
            2_i32.pow((overlap.len() - 1) as u32)
        });
        num_wins_raw.push(overlap.len() as i32);
    }

    println!("num_wins: {:?}", num_wins);
    println!("Total wins: {}", num_wins.iter().sum::<i32>());

    // Part 2:
    // Every matching number causes you to get that many scratchcards, below the current one

    // dbg!(num_wins_raw.clone());

    // 1 for every card
    let mut num_scratchcards = vec![1; num_wins.len()];

    for (i, num) in num_wins_raw.iter().enumerate() {
        let curr_overlaps = num;

        let curr_scratchcards = num_scratchcards[i];

        let start = i + 1usize;
        let end = i + *curr_overlaps as usize + 1;
        let end = min(end, num_scratchcards.len());

        // dbg!(start, end, curr_overlaps);

        // Add the scratchcards to all the cards below this one
        for j in start..end {
            num_scratchcards[j] += curr_scratchcards;
        }

        // dbg!(num_scratchcards.clone());
    }

    println!("num_scratchcards: {:?}", num_scratchcards);
    println!(
        "Total scratchcards: {}",
        num_scratchcards.iter().sum::<i32>()
    );
}
