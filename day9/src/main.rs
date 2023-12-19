fn process_line(line: &str) -> i128 {
    // println!("{:?}", line);
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
            diff += d;
        }

        right.push(d);
        // println!("{:?}", new_line);
        line = new_line;
    }

    right.reverse();
    // println!("{:?}", &right);

    let res = right.iter().fold(0, |a, x| a + x);
    // println!("{}", &res);
    res
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Could not read file!");
    let contents: Vec<_> = contents.lines().collect();
    // println!("{:?}", contents);
    // process_line(contents[2]);

    let res: i128 = contents.iter().map(|x| process_line(x)).sum();
    println!("{:?}", res);


















}
