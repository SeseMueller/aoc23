fn main() {

    // Start timer
    let start = std::time::Instant::now();
    let text = include_str!("TEMP.txt");
    // let text = include_str!("TEMP2.txt");


    // Today we have to construct a modified pascals sequence

    // For that, we have to calculate how "deep" the sequence is
    // so how many iterations of calculating differences of neighbors
    // until everything is 0. 

    // Then we can use pascals triangle to calculate the result.


    let mut result_sum = 0;
    let mut result_sum2 = 0;

    // Precompute pascals triangle
    let big_triangle = pascal_triangle(20);
    
    for line in text.lines() {
        let nums = line.split_whitespace().map(|x| x.parse::<i128>().unwrap()).collect::<Vec<_>>();

        let mut depth = 0;
        let mut last = nums.clone();

        loop {
            let mut new = Vec::new();
            for i in 0..last.len()-1 {
                new.push(last[i+1] - last[i]);
            }

            depth += 1;

            if new.iter().all(|&x| x == 0) {
                break;
            }

            last = new;

            // println!("{:?}", last);
            // Failsafe
            if depth > 10 * nums.len() {
                panic!("Too deep! {}", line);
            }
        }

        // println!("Depth: {}", depth);

        // let triangle = &pascal_triangle(depth+1)[depth];
        let triangle = &big_triangle[depth];

        // Invert every second element
        let mut triangle = triangle.iter().enumerate().map(|(i, &x)| if (triangle.len() - i) % 2 == 0 { x } else { -x }).collect::<Vec<_>>();

        // remove the last element
        triangle.pop();

        // println!("{:?}", triangle);

        // Overlay the elements over the last elements of the input
        // calculate the product of each pair and sum them up

        let mut sum = 0i128;
        for i in 0..triangle.len() {
            sum += triangle[i] * nums[nums.len()-triangle.len()+i];
        }

        // println!("Sum: {}", sum);

        result_sum += sum;

        // Part 2

        //Extrapolate backwards instead.

        // let triangle = &pascal_triangle(depth+1)[depth];
        let triangle = &big_triangle[depth];

        // Invert every second element
        let mut triangle = triangle.iter().enumerate().map(|(i, &x)| if i % 2 != 0 { x } else { -x }).collect::<Vec<_>>();

        // remove the first element
        triangle.remove(0);

        // println!("{:?}", triangle);

        // Overlay the elements over the first elements of the input
        // calculate the product of each pair and sum them up

        let mut sum = 0i128;
        for i in 0..triangle.len() {
            sum += triangle[i] * nums[i];
        }

        // println!("Sum: {}", sum);

        result_sum2 += sum;
    }

    println!("Result: {}", result_sum);
    println!("Result2: {}", result_sum2);

    // Print elapsed time in µs
    println!("Elapsed: {}µs", start.elapsed().as_micros());
    // Wow! 600µs! 
}

fn pascal_triangle(depth: usize) -> Vec<Vec<i128>> {
    let mut result: Vec<Vec<i128>> = Vec::new();

    for i in 0..depth {
        let mut row = Vec::new();
        for j in 0..i+1 {
            if j == 0 || j == i {
                row.push(1);
            } else {
                row.push(result[i-1][j-1] + result[i-1][j]);
            }
        }
        result.push(row);
    }

    result
}
