fn main() {
    
   let text = include_str!("TEMP.txt");
//    let text = include_str!("TEMP2.txt");

   let games = text.lines()
       .into_iter()
       .map(|line| match line.split(":").clone().collect::<Vec<&str>>().as_slice() {
              [a,b] => (a.to_owned(),b.to_owned()),
              _ => panic!("Unexpected line: {}", line),
         })
         .map(|(a,b)| (a.split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap(), 
         
         
         b.split(";").into_iter().map(|v| v.split(",").collect::<Vec<&str>>())))
       // Game id; X COLOR, Y COLOR, Z COLOR; A COLOR, B COLOR, …
       // Map to (Game id, [[X COLOR, Y COLOR, Z COLOR], [A COLOR, B COLOR, …]])
         .map(|(a,b)| (a, b.clone().into_iter().map(|b_inner|  b_inner.iter().map(|x| match x.trim().split(" ").clone().collect::<Vec<&str>>().as_slice() {
             [b,c] => {
                let num = b.parse::<i32>().unwrap();
                let color = c.to_owned();
                (num, color)
             },
                _ => panic!("Unexpected line: {:?}", b),
         }).collect::<Vec<(i32, &str)>>()).collect::<Vec<Vec<(i32, &str)>>>()))
        .collect::<Vec<(i32, Vec<Vec<(i32, &str)>>)>>();

    let mut game_id_sum = 0;

    let mut power_sum = 0;

    for (game_id, colors) in games {
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        let mut min_red = 100000;
        let mut min_blue = 100000;
        let mut min_green = 100000;

        for color in colors.iter() {
            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;
            for (num, color) in color {
                match *color {
                    "red" => red += num,
                    "blue" => blue += num,
                    "green" => green += num,
                    _ => panic!("Unexpected color: {}", color),
                }
            }
            if red > max_red {
                max_red = red;
            }
            if blue > max_blue {
                max_blue = blue;
            }
            if green > max_green {
                max_green = green;
            }

            if red < min_red {
                min_red = red;
            }
            if blue < min_blue {
                min_blue = blue;
            }
            if green < min_green {
                min_green = green;
            }
        }

        if max_red <=12 && max_blue <= 14 && max_green <= 13 {
            game_id_sum += game_id;
        }

        power_sum += max_red * max_blue * max_green;
    }

    println!("Game id sum: {}", game_id_sum);
    println!("Power sum: {}", power_sum);

}
