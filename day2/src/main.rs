fn part1() {
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    let mut valid_sum = 0;

    let file_path = "input.txt";
    let contents = std::fs::read_to_string(file_path).expect("Unable to read file!");

    for line in contents.lines() {
        println!("{line}");
        let mut valid = true;
        let info: Vec<&str> = line.split(":").collect();
        let id: u32 = info[0].strip_prefix("Game ").unwrap().parse().unwrap();
        for game in info[1].split(";") {
            println!("{:?}", game);

            for round in game.split(",") {
                let round = round.trim();
                println!("{:?}", round);
                let elements: Vec<&str> = round.split(" ").collect();

                let n: i32 = elements[0].parse().unwrap();
                match elements[1] {
                    "red" => {
                        if n > red_max {
                            valid = false
                        }
                    }
                    "green" => {
                        if n > green_max {
                            valid = false
                        }
                    }
                    "blue" => {
                        if n > blue_max {
                            valid = false
                        }
                    }
                    _ => println!("Invalid color!"),
                }
            }
        }
        println!("{id}");
        if valid {
            valid_sum += id
        }
    }
    println!("valid sum: {}", valid_sum);
}

fn part2() {
    let mut red_max = 0;
    let mut green_max = 0;
    let mut blue_max = 0;
    let mut power_sum = 0;

    let file_path = "input.txt";
    let contents = std::fs::read_to_string(file_path).expect("Unable to read file!");

    for line in contents.lines() {
        println!("{line}");
        let info: Vec<&str> = line.split(":").collect();
        let id: u32 = info[0].strip_prefix("Game ").unwrap().parse().unwrap();
        red_max = 0;
        green_max = 0;
        blue_max = 0;
        for game in info[1].split(";") {
            for round in game.split(",") {
                let round = round.trim();
                let elements: Vec<&str> = round.split(" ").collect();

                let n: i32 = elements[0].parse().unwrap();
                match elements[1] {
                    "red" => {
                        if n > red_max {
                            red_max = n
                        }
                    }
                    "green" => {
                        if n > green_max {
                            green_max = n
                        }
                    }
                    "blue" => {
                        if n > blue_max {
                            blue_max = n
                        }
                    }
                    _ => println!("Invalid color!"),
                }
            }
        }
        println!("min {} red {} green {} blue", red_max, green_max, blue_max);
        power_sum += red_max * green_max * blue_max;
    }
    println!("power sum: {}", power_sum);
}

fn main() {
    part2();
}
