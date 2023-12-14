
struct round {
    hand: &str,
    bet: u128,
}

enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn better_same_rank(hand1: &str, hand2: &str) {

    


}

fn main() {

    let contents = std::fs::read_to_string("input.txt").expect("Unable to read file!");

}
