use std::collections::HashMap;
use num::integer;

fn parse_line(line: &str) -> String {
    line.chars()
        .filter(|&x| x != '(' && x != ')' && x != ',' && x != '=')
        .collect::<String>()
}

fn part1(turns: Vec<char>, mappings: HashMap<&str, (&str, &str)>) {
    let mut key = &"AAA";
    let mut i = 0;
    let mut counter = 0;

    while key != &"ZZZ" {
        let turn = turns[i];
        let (a, b) = mappings.get(key).expect("Unable to locate key {key}!");
        key = match turn {
            'L' => a,
            'R' => b,
            _ => panic!("Unknown turn given {}!", turns[i]),
        };

        counter += 1;
        i = if i >= turns.len() - 1 { 0 } else { i + 1 };
    }

    println!("result: {:?}", counter);
}

fn all_end_with_z(keys: &Vec<&str>) -> bool {
    keys.len() == keys.iter().filter(|x| x.chars().nth(2) == Some('Z')).count()
}

fn part2(turns: Vec<char>, mappings: HashMap<&str, (&str, &str)>) {
    let mut keys: Vec<_> = mappings
        .keys()
        .filter(|x| x.chars().nth(2).unwrap() == 'A')
        .collect();

    let mut i = 0;
    let mut counter: u128 = 0;
    let l = keys.len();

    while l != keys.iter().filter(|&x| x.ends_with("Z")).count() {
        // println!("{counter} -> {:?}", keys);

        let turn = turns[i];
        for j in 0..keys.len() {
            let (a, b) = mappings.get(keys[j]).expect("Unable to locate key {key}!");
            keys[j] = match turn {
                'L' => &a,
                'R' => &b,
                _ => panic!("Unknown turn given {}!", turns[i]),
            };
            if keys[j].ends_with("Z") {
                println!("{counter} -> {:?}", keys);
            }
        }

        counter += 1;
        i = (i + 1) % turns.len();
        //i = if i >= turns.len() - 1 { 0 } else { i + 1 };
    }
    
    // After 3 hours of running >:/
    // 3609894725 -> ["VBN", "XPD", "MSL", "CFH", "MSQ", "BFH"]
    println!("result: {:?}", counter);
}

fn min_steps_to_z(key: &str, turns: &Vec<char>, mappings: &HashMap<&str, (&str, &str)>) -> u128 {
    let mut key = key;
    let mut i = 0;
    let mut counter: u128 = 0;

    while !key.ends_with("Z") {
        let turn = turns[i];
        let (a, b) = mappings.get(key).expect("Unable to locate key {key}!");
        key = match turn {
            'L' => a,
            'R' => b,
            _ => panic!("Unknown turn given {}!", turns[i]),
        };

        counter += 1;
        i = if i >= turns.len() - 1 { 0 } else { i + 1 };
    }
    counter
}

fn part2b(turns: Vec<char>, mappings: HashMap<&str, (&str, &str)>) {
    let keys: Vec<_> = mappings
        .keys()
        .filter(|x| x.chars().nth(2).unwrap() == 'A')
        .collect();

    let keys: Vec<_> = keys.iter().map(|x| min_steps_to_z(x, &turns, &mappings)).collect();
    println!("{:?}", keys);
    
    let keys = keys.iter().fold(1, |acc, x| integer::lcm(*x, acc));
    println!("{:?}", keys);


}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Could not read file!");
    let contents: Vec<_> = contents.lines().collect();

    let turns: Vec<char> = contents[0].chars().collect();

    let contents: Vec<_> = contents[2..].iter().map(|x| parse_line(x)).collect();
    let contents: Vec<_> = contents
        .iter()
        .map(|x| x.split(" ").filter(|y| *y != "").collect::<Vec<_>>())
        .collect();

    let mappings = contents
        .iter()
        .map(|x| (x[0], (x[1], x[2])))
        .collect::<HashMap<_, _>>();

    // part1(turns, mappings);
    part2b(turns, mappings);
}
