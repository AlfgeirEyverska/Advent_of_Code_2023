use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct PotentialPartNumber {
    num: String,
    row: usize,
    start: usize,
    end: usize,
}

impl PotentialPartNumber {
    fn part_number(&self) -> u32 {
        let pn: u32 = self.num.parse().expect("Unable to parse the part number!");
        pn
    }

    fn surrounding_chars(&self, input: &Vec<&str>) -> Vec<char> {
        let mut ret: Vec<char> = vec![];
        let same_row: Vec<char> = input[self.row].chars().collect();
        let left = if self.start > 0 {
            ret.push(same_row[self.start - 1]);
            self.start - 1
        } else {
            self.start
        };

        let right = if self.end < input[0].len() - 1 {
            ret.push(same_row[self.end]);
            self.end
        } else {
            self.end - 1
        };
        if self.row > 0 {
            let prev_row: Vec<char> = input[self.row - 1].chars().collect();
            for i in left..=right {
                ret.push(prev_row[i]);
            }
        }
        if self.row < input.len() - 1 {
            let next_row: Vec<char> = input[self.row + 1].chars().collect();
            for i in left..=right {
                ret.push(next_row[i]);
            }
        }
        ret
    }

    fn is_valid(&self, input: &Vec<&str>) -> bool {
        let mut valid = false;
        let special = vec!['@', '#', '$', '%', '&', '*', '+', '-', '=', '/'];
        for c in self.surrounding_chars(input).iter() {
            if special.contains(c) {
                valid = true;
                break;
            }
        }
        valid
    }

    fn adjacent_stars(&self, input: &Vec<&str>) -> Vec<(usize, usize)> {
        let mut ret: Vec<(usize, usize)> = vec![];
        let same_row: Vec<char> = input[self.row].chars().collect();
        let left = if self.start > 0 {
            if same_row[self.start - 1] == '*' {
                ret.push((self.row, self.start - 1))
            }
            self.start - 1
        } else {
            self.start
        };
        let right = if self.end < input[0].len() - 1 {
            if same_row[self.end] == '*' {
                ret.push((self.row, self.end))
            }
            self.end
        } else {
            self.end - 1
        };
        if self.row > 0 {
            let prev_row: Vec<char> = input[self.row - 1].chars().collect();
            for i in left..=right {
                if prev_row[i] == '*' {
                    ret.push((self.row - 1, i))
                }
            }
        }
        if self.row < input.len() - 1 {
            let next_row: Vec<char> = input[self.row + 1].chars().collect();
            for i in left..=right {
                if next_row[i] == '*' {
                    ret.push((self.row + 1, i))
                }
            }
        }
        ret
    }
}

fn main() {
    let file_path =
        "/Users/ty/Library/Mobile Documents/com~apple~CloudDocs/Advent_of_Code/day3b/input.txt";
    let contents = std::fs::read_to_string(file_path).expect("Unable to read file!");
    let contents: Vec<&str> = contents.lines().collect();
    let mut ppns: Vec<PotentialPartNumber> = vec![];

    let re = Regex::new(r"[0-9]+").unwrap();

    for (row, line) in contents.iter().enumerate() {
        for cap in re.find_iter(line) {
            ppns.push(PotentialPartNumber {
                num: cap.as_str().to_string(),
                row: row,
                start: cap.start(),
                end: cap.end(),
            });
        }
    }
    
    let mut gears = HashMap::new();

    for p in ppns.iter() {
        let astars = p.adjacent_stars(&contents);
        for a in &astars {
            let key = format!("{:?}", a);
            gears
                .entry(key)
                .and_modify(|vek: &mut Vec<u32>| vek.push(p.part_number()))
                .or_insert(vec![p.part_number()]);
        }
    }

    let results: u128 = gears
        .iter()
        .filter(|(k, v)| v.len() == 2)
        .map(|(k, v)| v.iter().fold(1, |acc, x| acc * *x as u128))
        .sum();
    println!("{}", results);
}
