use std::{fs::File, io::Read};


fn main() {
    let mut file = File::open("src/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    let mut nums_vec : Vec<Vec<i32>> = vec![];
    for line in contents.lines() {
        let parts = line.split_whitespace();
        let mut nums : Vec<i32> = vec![];
        for part in parts{
            nums.push(part.parse::<i32>().unwrap());
        }
        nums_vec.push(nums);
    }

    //Part 1
    let part1_count = nums_vec.iter().filter(|r| is_safe(r)).count();

    // Part 2
    let part2_count = nums_vec.iter().filter(|r| is_safe_2(r)).count();

    println!("The solution to the first part is: {}", part1_count);
    println!("The solution to the second part is: {}", part2_count);
}

fn is_safe(report: &Vec<i32>) -> bool {
    let increasing = report.windows(2).all(|w| (1..=3).contains(&(w[1] - w[0])));
    let decreasing = report.windows(2).all(|w| (1..=3).contains(&(w[0] - w[1])));
    increasing || decreasing
}

fn is_safe_2(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut report_2 = report.clone();
        report_2.remove(i);
        if is_safe(&report_2) {
            return true;
        }
    }
    false
}
