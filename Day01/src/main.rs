use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("src/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap(); 
    
    let mut left: Vec<i32> = vec![]; 
    let mut right: Vec<i32> = vec![]; 
    for line in contents.lines() {
        let mut halves = line.split_whitespace();
        left.push(halves.next().unwrap().parse::<i32>().unwrap());
        right.push(halves.next().unwrap().parse::<i32>().unwrap());
    }
    
    //Part 1

    let mut left_copy = left.clone();
    let mut right_copy = right.clone();

    left_copy.sort();
    right_copy.sort();
    let mut sum1 = 0;
    for _ in 0..left_copy.len() {
        sum1 += (left_copy.pop().unwrap() - right_copy.pop().unwrap()).abs();
    }

    //Part 2

    let mut sum2 = 0;
    for &l in &left {
        let mut factor = 0;
        for &r in &right{
            if l==r {
                factor += 1;
            }
        }
        sum2 += factor * l;
    }

    println!("The solution to the first question is {}",sum1);
    println!("The solution to the second question is {}",sum2);
}
