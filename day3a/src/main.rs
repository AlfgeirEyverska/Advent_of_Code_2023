use regex::Regex;

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
}

fn main() {
    let file_path = "input.txt";
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

    /*
        let nums: Vec<PotentialPartNumber> = contents.iter().enumerate().map(|(i, x)| {
            re.find_iter(x).map(|y| PotentialPartNumber {
                num: y.as_str().to_string(),
                row: i,
                start: y.start(),
                end: y.end(),
            });
        }).flatten();
    */
    let s: u32 = ppns
        .iter()
        .filter(|x| x.is_valid(&contents))
        .map(|x| x.part_number())
        .sum();

    println!("{}", s);
}
