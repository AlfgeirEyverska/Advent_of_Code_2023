fn process_line(line: &str) -> i128 {
    println!("##########################################################################################################");
    println!("##########################################################################################################");
    println!("##########################################################################################################");
    println!("{:?}", line);
    let mut line: Vec<_> = line
        .split(" ")
        .map(|x| {
            x.parse::<i128>()
                .expect("Could not parse as int!")
        })
        .collect();

    let mut right = Vec::new();
    right.push(line[line.len() - 1]);

    let mut diff = 37;
    while diff != 0 {
        diff = 0;
        let mut d = 0;
        let mut new_line = Vec::new();

        for i in 0..line.len() - 1 {
            d = line[i + 1] - line[i];
            new_line.push(d);
            diff += d*d;
        }

        right.push(d);
        println!("{:?}", new_line);
        line = new_line;
    }

//    right.reverse();
    println!("RIGHT: {:?}", &right);
    let res = right.iter().sum();
//    let res = right.iter().fold(0, |a, x| a + x);
    println!("result: {}", &res);
    res
}

fn process_line_backwards(line: &str) -> i128 {
    println!("##########################################################################################################");
    println!("##########################################################################################################");
    println!("##########################################################################################################");
    println!("{:?}", line);
    let mut line: Vec<_> = line
        .split(" ")
        .map(|x| {
            x.parse::<i128>()
                .expect("Could not parse as int!")
        })
        .collect();

    let mut left = Vec::new();
    left.push(line[0]);

    let mut diff = 37;
    while diff != 0 {
        diff = 0;
        let mut d = 0;
        let mut new_line = Vec::new();

        for i in 0..line.len() - 1 {
            d = line[i + 1] - line[i];
            if i == 0 {
                left.push(d);
            }
            new_line.push(d);
            diff += d*d;
        }

        println!("{:?}", new_line);
        line = new_line;
    }

    left.reverse();
    println!("LEFT: {:?}", &left);
//    let res: i128 = left.iter().sum();
//    let res: i128 = line[0] - res;
    let res = left.iter().fold(0, |a, x| x - a);
    println!("result: {}", &res);
res
}


fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Could not read file!");
    let contents: Vec<_> = contents.lines().collect();
    // println!("{:?}", contents);
    // process_line(contents[2]);

    let res: i128 = contents.iter()
                            .map(|x| process_line_backwards(x))
                            .sum();
    println!("FINAL RESULT:  {:?}", res);

}
