fn count_winners(card: &str) -> u32 {

    let card_info: Vec<&str> = card.split(":").collect();
    let card_number = card_info[0].strip_prefix("Card").unwrap().trim();
    let game_numbers: Vec<&str> = card_info[1].split("|").collect();

    let winning_numbers: Vec<u32> = game_numbers[0]
        .split(" ")
        .filter(|x| x != &"")
        .map(|x| x.parse::<u32>().expect("Cannot parse int from string!"))
        .collect();

    let player_numbers: Vec<u32> = game_numbers[1]
        .split(" ")
        .filter(|x| x != &"")
        .map(|x| x.parse::<u32>().expect("Cannot parse int from string!"))
        .collect();

    let winners: usize = winning_numbers
        .iter()
        .filter(|x| player_numbers.contains(x))
        .count();

    // println!("{}", winners);
    winners as u32
}

fn part1(contents: &Vec<&str>) -> u128 {

    let result:u128 = contents.iter().map(|x| count_winners(&x)).filter(|x| *x != 0).map(|x| 2u128.pow(x - 1)).sum();
    println!("result: {}", result);
    result

}

fn part2(contents: &Vec<&str>) -> u128 {

    /*
     * this is interesting, i need to process the first set of cards and all subsequent sets of
     * cards. Any 
     * */

    0u128;
}


fn main() {
    let file_path =
        "/Users/ty/Library/Mobile Documents/com~apple~CloudDocs/Advent_of_Code/day4/input.txt";
    let contents = std::fs::read_to_string(file_path).expect("Unable to read file!");
    let contents: Vec<&str> = contents.lines().collect();


    part2(&contents);




}
