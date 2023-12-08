fn main() {
    let text = include_str!("TEMP.txt");
    //  let text = include_str!("TEMP2.txt");

    let mut m1 = possible_distances(57);m1.retain(|x| x > &291);
    let mut m2 = possible_distances(72);m2.retain(|x| x > &1172);
    let mut m3 = possible_distances(69);m3.retain(|x| x > &1176);
    let mut m4 = possible_distances(92);m4.retain(|x| x > &2026);

     // product of lengths
        let prod = m1.len() * m2.len() * m3.len() * m4.len();

    println!("prod: {}", prod);


    let fix_max = 57726992;
    let fix_dist = 291117211762026;
    // let fix_max = 71530;
    // let fix_dist = 940200;

    let mut max = fix_max;
    let mut min = 0;
    
    // Bisect until we find the first time it's more than fix_dist
    loop {
        let mid = (max + min) / 2;
        let dist = distance_travelled(mid, fix_max);
        // dbg!(dist);

        if dist > fix_dist {
            max = mid;
        } else {
            min = mid;
        }

        if max - min <= 1 {
            break;
        }
    }

    println!("min: {}", min);
    let result_min = min;

    // Now, bisect until we find the first time it's more than fix_dist

    let mut max = fix_max;
    let mut min = 0;
    

    loop {
        let mid = (max + min) / 2;
        let dist = distance_travelled(mid, fix_max);

        if dist < fix_dist {
            max = mid;
            // dbg!(max);
        } else {
            min = mid;
            // dbg!(min);
        }

        if max - min <= 1 {
            break;
        }
    }

    println!("max: {}", max);
    let result_max = max;

    println!("result: {}", result_max - result_min - 1); // 56709230
}


fn distance_travelled(b_timer: i64, race_timer: i64) -> i64 {
    // The boat travels for (racetimer-b_timer) seconds at b_timer speed

    let travel_time = race_timer - b_timer;
    let travel_distance = travel_time * b_timer;
    travel_distance
}

fn possible_distances(race_timer: i64) -> Vec<i64>{
    (0..race_timer)
    .map(|x| distance_travelled(x, race_timer))
    .collect::<Vec<_>>()
}