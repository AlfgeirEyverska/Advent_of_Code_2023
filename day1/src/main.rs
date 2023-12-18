fn main() {
    let contents = std::fs::read_to_string("aaron.txt").expect("Unable to read file!");

    let acceptable = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let nums = std::collections::HashMap::from([
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ]);

    let mut answer = 0u32;
    for line in contents.lines() {
        let mut line = line.to_string();
        for (word, num) in &nums {
            line = line.replace(word, num);
        }
        // println!("{}", line);
        let res: Vec<char> = line.chars().filter(|x| acceptable.contains(x)).collect();
        // println!("{:?}", res);
        let cal_val = format!("{}{}", res[0], res[res.len() - 1]);
        let cal_val: u32 = cal_val.parse().unwrap();
        // println!("{}", cal_val);
        answer += cal_val;
        println!("{}", cal_val);
    }
    println!("result: {answer}");
}
