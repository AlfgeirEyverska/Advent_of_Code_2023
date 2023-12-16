use std::collections::HashMap;

#[derive(Debug)]
struct Round {
    hand_strength: u32,
    bet: u128,
    //hand_type: HandType,
}

impl Round {
    fn from(line: &str) -> Round {
        let split_line: Vec<_> = line.split(" ").collect();
        Round {
            hand_strength: strength(split_line[0]),
            bet: split_line[1].parse::<u128>().expect("Could not parse bet!"),
            //hand_type: hand_type(split_line[0]),
        }
    }
}

#[derive(Debug)]
enum HandType {
    FiveOfKind = 0xF,
    FourOfKind = 0xE,
    FullHouse = 0xD,
    ThreeOfKind = 0xC,
    TwoPair = 0xB,
    OnePair = 0xA,
    HighCard = 0x9,
}

fn strength(hand: &str) -> u32 {
    let mut cards = String::new();
    for card in hand.chars() {
        let c = match card {
            'A' => 'E',
            'K' => 'D',
            'Q' => 'C',
            //'J' => 'B',
            'T' => 'A',
            '9' => '9',
            '8' => '8',
            '7' => '7',
            '6' => '6',
            '5' => '5',
            '4' => '4',
            '3' => '3',
            '2' => '2',
            'J' => '1',
            _ => panic!("Unknown card detected!"),
        };
        cards.push(c);
    }

    let hand_type = hand_type(hand);
    let strength = format!("{:X}{}", hand_type as u32, cards);
    let strength = u32::from_str_radix(&strength, 16).unwrap();

    //    println!("hand: {:?}", hand);
    //    println!("strn: {:?}", strength);
    strength
}

fn hand_type_a(hand: &str) -> HandType {
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

fn hand_type(hand: &str) -> HandType {
    let mut cards = HashMap::new();
    let mut jokers = 0;

    for item in hand.chars() {
        if item == 'J' {
            jokers += 1;
        } else {
            cards.entry(item).and_modify(|val| *val += 1).or_insert(1);
        }
    }

    let mut cards: Vec<_> = cards.into_values().collect();
    cards.sort();
    cards.reverse();

    if jokers == 5 {
        cards = vec![5];
        jokers = 0;
    }
    let mut i = 0;
    while jokers > 0 {
        if cards[i] < 5 {
            cards[i] += 1;
            jokers -= 1;
        } else {
            i += 1;
        }
    }

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
    /*
    let contents = String::from(
    "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483");
    */
    let mut hands = Vec::new();
    for line in contents.lines() {
        let r = Round::from(line);
        hands.push(r);
    }

    hands.sort_by(|x, y| y.hand_strength.cmp(&x.hand_strength));
    hands.reverse();
    let mut res = 0;
    for (i, h) in hands.iter().enumerate() {
        res += h.bet * (i as u128 + 1);
    }

    println!("result: {}", res);
}
