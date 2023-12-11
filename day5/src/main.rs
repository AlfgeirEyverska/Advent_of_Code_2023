use itertools::Itertools;

#[derive(Debug)]
struct AlmanacMap {
    dest_start: u32,
    src_start: u32,
    range_len: u32,
}

impl AlmanacMap {
    fn new(line: &str) -> AlmanacMap {
        let items: Vec<u32> = line
            .split(" ")
            .map(|x| x.parse::<u32>().expect("Could not parse the line!"))
            .collect();
        AlmanacMap {
            dest_start: items[0],
            src_start: items[1],
            range_len: items[2],
        }
    }

    fn contains(&self, num: u32) -> bool {
        num >= self.src_start && num <= self.src_start + self.range_len
    }
}

fn lookup(num: u32, this: &Vec<AlmanacMap>) -> u32 {
    let mut found = false;
    let mut result: u32 = 0;
    //    println!("LOOKING FOR {}", num);
    for (index, item) in this.iter().enumerate() {
        //        println!("{} : {:?}", index, item);
        if item.contains(num) {
            found = true;
            result = item.dest_start + (num - item.src_start);
            break;
        }
    }
    if !found {
        result = num;
    }
    result
}

fn location(num: u32, mappings: &Vec<Vec<AlmanacMap>>) -> u32 {
    let mut next_target = num;
    for stage in mappings {
        next_target = lookup(next_target, &stage);
    }
    next_target
}

fn result(contents: &Vec<&str>) {
    let seeds: Vec<&str> = contents[0].split(":").map(|x| x.trim()).collect();
    let seeds: Vec<u32> = seeds[1]
        .split(" ")
        .map(|x| x.parse::<u32>().expect("Could not parse int!"))
        .collect();
    println!("{:?}", seeds);
    println!("Growing seeds");
// Part 2 requires special treatment of the seeds from part 2 above
    let seeds: Vec<_> = seeds
        .iter()
        .tuples()
        .map(|(&x, &y)| (x..x + y).collect::<Vec<u32>>())
        .collect();
    for seed in &seeds {
        println!("{:?}", seed);
    }
        

    let seeds: Vec<_> = seeds.iter().flatten().collect();
// End of the part 2 additions
    
/*
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
        .map(|x| location(*x.clone(), &mappings))
        .min()
        .unwrap();
    println!("{:?}", result);
    */
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Unable to read file!");
    let contents: Vec<&str> = contents.lines().filter(|x| *x != "").collect();

    result(&contents);
}
