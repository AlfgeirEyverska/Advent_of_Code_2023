fn go_north(contents: Vec<Vec<char>>, row: usize, col: usize, path: &mut Vec<(usize, usize)>) {
    let row = row - 1;
    let c = contents[row][col];
    path.push((row, col));
    match c {
        '7' => go_west(contents, row, col, path),
        '|' => go_north(contents, row, col, path),
        'F' => go_east(contents, row, col, path),
        _ =>  return,
    }
}

fn go_east(contents: Vec<Vec<char>>, row: usize, col: usize, path: &mut Vec<(usize, usize)>) {
    let col = col + 1;
    let c = contents[row][col];
    path.push((row, col));
    match c {
        '7' => go_south(contents, row, col, path),
        '-' => go_east(contents, row, col, path),
        'J' => go_north(contents, row, col, path),
        _ =>  return,
    }
}

fn go_south(contents: Vec<Vec<char>>, row: usize, col: usize, path: &mut Vec<(usize, usize)>) {
    let row = row + 1;
    let c = contents[row][col];
    path.push((row, col));
    match c {
        'J' => go_west(contents, row, col, path),
        '|' => go_south(contents, row, col, path),
        'L' => go_east(contents, row, col, path),
        _ =>  return,
    }
}

fn go_west(contents: Vec<Vec<char>>, row: usize, col: usize, path: &mut Vec<(usize, usize)>) {
    let col = col - 1;
    let c = contents[row][col];
    path.push((row, col));
    match c {
        'L' => go_north(contents, row, col, path),
        '-' => go_west(contents, row, col, path),
        'F' => go_south(contents, row, col, path),
        _ =>  return,
    }
}

fn find_S(contents: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..contents.len() {
        for j in 0..contents[i].len() {
            if contents[i][j] == 'S' {
                return (i, j);
            }
        }
    }
    panic!("S not found in input!");
}

fn replace_s(path: &Vec<(usize, usize)>) -> char {
    let (sx, sy) = path[path.len()-1];
    let (sx, sy) = (sx as i32, sy as i32);
    let (ax, ay) = path[0];
    let (ax, ay) = (ax as i32, ay as i32);
    let (bx, by) = path[path.len()-2];
    let (bx, by) = (bx as i32, by as i32);
    
//    println!("a: {:?}",(ax - sx, ay - sy));
//    println!("b: {:?}",(bx - sx, by - sy));
//    println!("differences: {:?}",(ax - sx + bx - sx, ay - sy + by - sy));
    let res = match (ax - sx + bx - sx, ay - sy + by - sy) {
        (-1, -1) => 'J',
        (-1, 1) => 'L',
        (1, -1) => '7',
        (1, 1) => 'F',
        (0, 0) => if (ax - sx + bx - sx) == 0 {'-'} else {'|'},
        _ => panic!("Could not figure out substitution!"),
    };
    println!("Substitution:  {:?}", res);
    res
}

fn part1(contents: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    
    let rows = contents.len();
    let cols = contents[0].len();
    println!("Rows: {:?}\nCols: {:?}", rows, cols);
    
    let (r, c) = find_S(&contents);
    println!("S is at:\nRow: {:?} -- Col: {:?}", r, c);
    
    let mut path: Vec<(usize, usize)> = vec![];
//    Instead of checking in the code, I just eyeball it
    go_east(contents, r, c, &mut path);
//    go_south(contents, r, c, &mut path);
    println!("Path:\n{:?}", path);
    
    let res = path.len()/2;
    println!("Part 1 Result:  {:?}", res);
    
    path
}

fn _part2(mut contents: Vec<Vec<char>>) -> usize {
    
    let path = part1(contents.clone());
    println!("Path:\n{:?}", path);
//    matrix of bools to keep track of what is inside
    let mut insides = vec![vec![false; contents[0].len()]; contents.len()];
    let s = replace_s(&path);
    let (sx, sy) = path[path.len()-1];
    contents[sx][sy] = s;
    
//    While searching up look for 7 or F
//    While searching down look for J or L
//    While searching left look for F or L
//    While searching right look for J or 7
    
    for i in 0..contents.len() {
        for j in 0..contents[i].len() {
//            Ensure it is not part of the loop
            if path.contains(&(i, j)) { continue }
            
//            let mut first = '0';
//            Check up
            let up_odd: bool;
            let mut counter = 0;
            for ii in 0..i {
                if path.contains(&(ii, j)) { counter += 1 };
            }
            if counter % 2 == 1 { up_odd = true } else { up_odd = false }
            
//            Check down
            let down_odd: bool;
            let mut counter = 0;
            for ii in i..contents.len() {
                if path.contains(&(ii, j)) { counter += 1 };
            }
            if counter % 2 == 1 { down_odd = true } else { down_odd = false }
            
//            Check left
            let left_odd: bool;
            let mut counter = 0;
            for jj in 0..j{
                if path.contains(&(i, jj)) { counter += 1 };
            }
            if counter % 2 == 1 { left_odd = true } else { left_odd = false }
            
//            Check right
            let right_odd: bool;
            let mut counter = 0;
            for jj in j..contents[0].len() {
                if path.contains(&(i, jj)) { counter += 1 };
            }
            if counter % 2 == 1 { right_odd = true } else { right_odd = false }
            
//            Check all
            if up_odd && down_odd && left_odd && right_odd { println!("{:?}", (i, j))}
            insides[i][j] = up_odd && down_odd && left_odd && right_odd;
        }
    }
    
    for row in contents.iter() {
        println!("{:?}", row);
    }
    
    let mut counter = 0;
    for i in 0..insides.len() {
        println!("{:?}", insides[i]);
        for j in 0..insides[0].len() {
            if insides[i][j] { counter += 1 }
        }
    }
    println!("Part 2 Result:  {:?}", counter);
    counter
}

fn part2(contents: Vec<Vec<char>>) {
    /*
    After almost implementing my original solution, but stopping short of figuring out
    if I was running along a pipe instead of crossing over it, I looked it up and found
    these suggestions on the advent of code reddit.
    https://en.wikipedia.org/wiki/Shoelace_formula
    https://en.wikipedia.org/wiki/Pick%27s_theorem
    */
    let path = part1(contents.clone());
    
    let mut sum = 0;
    for i in 0..path.len()-1 {
        let (y1, x1) = path[i];
        let (y2, x2) = path[i+1];
        sum += x1 as i32 * y2 as i32 - x2 as i32 * y1 as i32;
    }
    let (y1, x1) = path[path.len()-1];
    let (y2, x2) = path[0];
    sum += x1 as i32 * y2 as i32 - x2 as i32 * y1 as i32;
    
    let area = sum / 2;
    let area = if area < 0 { -area } else { area };
    println!("Sum:  {:?}", sum);
    println!("Area:  {:?}", area);
    
    let b = path.len() as i32;
    println!("Boundary points:  {:?}", b);
    
    println!("Pick's Theorem:  area + 1 - b / 2");
    println!("Pick's Theorem:  {:?} + 1 - {:?} / 2", area, b);
    
    let interior = area + 1 - b/2;
    println!("Interior:  {:?}", interior);
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Could not read file!");
    let contents: Vec<Vec<char>> = contents.lines().map(|x| x.chars().collect()).collect();
    
    for row in contents.iter() {
        println!("{:?}", row);
    }
    
    part2(contents);
//    let path = part1(contents);
//    replace_s(&path);
    
}
