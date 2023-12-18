use std::collections::HashMap;

fn parse_line(line: &str) -> String {
    line.chars()
        .filter(|&x| x != '(' && x != ')' && x != ',' && x != '=')
        .collect::<String>()
}

fn part1 (turns: Vec<char>, mappings: HashMap<&str,(&str, &str)>) {
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

fn part2 (turns: Vec<char>, mappings: HashMap<&str,(&str, &str)>) {


    let paths: Vec<_> = mappings.keys().filter(|x| x.chars().get(2).unwrap() == 'A').collect();
    println!("Paths:\n{:?}", paths);



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

    for line in &mappings {
        println!("{:?}", line);
    }

    // part1(turns, mappings);
    part2(turns, mappings);



}
