use itertools::Itertools;

#[derive(Debug)]
struct AlmanacMap {
    dest_start: u64,
    src_start: u64,
    range_len: u64,
}

impl AlmanacMap {
    fn new(line: &str) -> AlmanacMap {
        let items: Vec<u64> = line
            .split(" ")
            .map(|x| x.parse::<u64>().expect("Could not parse the line!"))
            .collect();
        AlmanacMap {
            dest_start: items[0],
            src_start: items[1],
            range_len: items[2],
        }
    }

    fn contains(&self, num: u64) -> bool {
        num >= self.src_start && num <= self.src_start + self.range_len
    }
}

fn lookup(num: u64, section: &Vec<AlmanacMap>) -> u64 {
    /*
    let mut found = false;
    let mut result: u64 = 0;
    for item in section.iter() {
        if item.contains(num) {
            found = true;
            result = item.dest_start + (num - item.src_start);
            break;
        }
    }
    if !found {
        result = num;
    }
    */
    section
        .iter()
        .filter(|x| x.contains(num))
        .map(|x| x.dest_start + (num - x.src_start))
        .nth(0)
        .unwrap_or(num)
}

fn location(num: u64, mappings: &Vec<Vec<AlmanacMap>>) -> u64 {
    /*
       let mut next_target = num;
        for stage in mappings {
            next_target = lookup(next_target, &stage);
        }
    */
    mappings.iter().fold(num, |acc, next| lookup(acc, next))
}

fn result(contents: &Vec<&str>) {
    let seeds: Vec<&str> = contents[0].split(":").map(|x| x.trim()).collect();
    let seeds: Vec<u64> = seeds[1]
        .split(" ")
        .map(|x| x.parse::<u64>().expect("Could not parse int!"))
        .collect();

    // End of seeds setup for part 1
    /*
    let seeds: Vec<_> = seeds
        .iter()
        .tuples()
        .map(|(&x, &y)| (x..x + y).collect::<Vec<u64>>())
        .flatten()
        .collect();
    */

    let section_breaks = vec![
        "seed-to-soil map:",
        "soil-to-fertilizer map:",
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:",
    ];

    let section_breaks: Vec<usize> = section_breaks
        .iter()
        .map(|s| {
            contents
                .iter()
                .position(|x| x == s)
                .expect("Could not parse sections!")
        })
        .collect();
    let section_breaks = [section_breaks, vec![contents.len()]].concat();

    let mappings = vec![
        &contents[section_breaks[0] + 1..section_breaks[1] - 1],
        &contents[section_breaks[1] + 1..section_breaks[2] - 1],
        &contents[section_breaks[2] + 1..section_breaks[3] - 1],
        &contents[section_breaks[3] + 1..section_breaks[4] - 1],
        &contents[section_breaks[4] + 1..section_breaks[5] - 1],
        &contents[section_breaks[5] + 1..section_breaks[6] - 1],
        &contents[section_breaks[6] + 1..section_breaks[7] - 1],
    ];

    let mappings: Vec<Vec<AlmanacMap>> = mappings
        .iter()
        .map(|x| x.iter().map(|y| AlmanacMap::new(y)).collect())
        .collect();

    let result = seeds
        .iter()
        .map(|x| location(x.clone(), &mappings))
        .min()
        .unwrap();
    println!("{:?}", result);
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Unable to read file!");
    let contents: Vec<&str> = contents.lines().filter(|x| *x != "").collect();

    result(&contents);
}
