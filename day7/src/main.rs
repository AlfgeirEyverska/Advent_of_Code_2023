use std::collections::HashMap;

#[derive(Debug)]
struct Round {
    hand: String,
    bet: u128,
    hand_type: HandType,
}

#[derive(Debug)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Round {
    fn from(line: &str) -> Round {
        let split_line: Vec<_> = line.split(" ").collect();
        Round {
            hand: String::from(split_line[0]),
            bet: split_line[1].parse::<u128>().expect("Could not parse bet!"),
            hand_type: hand_type(split_line[0]),
        }
    }
}

fn strenght_of(card: char) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("Unknown card type encountered! {:?}", card),
    }
}

fn hand_type(hand: &str) -> HandType {
    let mut cards = HashMap::new();
    for item in hand.chars() {
        cards.entry(item).and_modify(|val| *val += 1).or_insert(1);
    }
    let mut cards: Vec<_> = cards.into_values().collect();
    cards.sort();
    cards.reverse();

    let key = cards.iter().fold(0, |acc, x| 10 * acc + x);

    match key {
        5 => HandType::FiveOfKind,
        41 => HandType::FourOfKind,
        32 => HandType::FullHouse,
        311 => HandType::ThreeOfKind,
        221 => HandType::TwoPair,
        2111 => HandType::OnePair,
        11111 => HandType::HighCard,
        _ => panic!("Unsupported hand type detected! {:?}", hand),
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Unable to read file!");
    let content = contents.lines().nth(0).unwrap();
    println!("{content}");

    let r = Round::from(&"12aaa 37");
    //let r = Round::from(content);

    println!("{:?}", r.hand_type);
}
