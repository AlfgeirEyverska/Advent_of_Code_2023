fn charged_distances(seconds: u128) -> Vec<u128> {
    /* if you charge for 0 you go 0 for n
     * if you charge n you go n for 0
     * if you charge m you go m for n - m
     */
    (0..=seconds).map(|x| x * (seconds - x)).collect()
}

fn num_winners(race: (u128, u128)) -> u128 {
    let (time, dist) = race;

    charged_distances(time)
        .iter()
        .filter(|&&x| x > dist)
        .count() as u128
}

fn part1() -> u128 {
    // let times_and_distances = vec![(7_u128, 9), (15, 40), (30, 200)];

    let times_and_distances = vec![(51_u128, 377), (69, 1171), (98, 1224), (78, 1505)];

    times_and_distances
        .iter()
        .map(|&x| num_winners(x))
        .reduce(|x, y| x * y)
        .unwrap()
}

fn part2() -> u128 {
    let time_and_distance = (51699878_u128, 377117112241505_u128);

    num_winners(time_and_distance)
}

fn main() {
    let result = part2();
    println!("{}", result);
}
