fn count_winners(card: &str) -> usize {
    let card_info: Vec<&str> = card.split(":").collect();
    let card_number = card_info[0].strip_prefix("Card").unwrap().trim();
    let game_numbers: Vec<&str> = card_info[1].split("|").collect();

    let winning_numbers: Vec<usize> = game_numbers[0]
        .split(" ")
        .filter(|x| x != &"")
        .map(|x| x.parse::<usize>().expect("Cannot parse int from string!"))
        .collect();

    let player_numbers: Vec<usize> = game_numbers[1]
        .split(" ")
        .filter(|x| x != &"")
        .map(|x| x.parse::<usize>().expect("Cannot parse int from string!"))
        .collect();

    let winners: usize = winning_numbers
        .iter()
        .filter(|x| player_numbers.contains(x))
        .count();

    winners as usize
}

fn part1(contents: &Vec<&str>) -> u32 {
    let result: u32 = contents
        .iter()
        .map(|x| count_winners(&x))
        .filter(|x| *x != 0)
        .map(|x| 2u32.pow(x as u32 - 1))
        .sum();
    result
}

fn part2(contents: &Vec<&str>) -> usize {
    let mut cards = vec![1; contents.len()];
    for (i, card) in contents.iter().enumerate() {
        let w = count_winners(&card);
        let n = cards[i];
        for item in &mut cards[i + 1..=i + w] {
            *item += n;
        }
    }
    cards.iter().sum()
}

fn main() {
    let file_path = "input.txt";
    let contents = std::fs::read_to_string(file_path).expect("Unable to read file!");
    let contents: Vec<&str> = contents.lines().collect();

    let result = part2(&contents);
    println!("{}", result);
}
