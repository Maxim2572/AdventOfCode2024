use std::{fs::File, io::Read, vec};

fn main() {
    let mut file = File::open("src/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Part 1
    let mut results :Vec<u128> = vec![];
    let mut numbers : Vec<Vec<u128>> = vec![];
    for line in contents.lines(){
        results.push(line.split(':').next().unwrap().parse().unwrap());

        let mut tmp: Vec<u128> = vec![]; 
        for num in line.split(':').nth(1).unwrap().split_whitespace() {
            tmp.push(num.parse().unwrap());
        }
        numbers.push(tmp);
    }

    let mut sum1 = 0;
    for i in 0..results.len() {
        if check(i as u128, &results, &numbers){
            sum1 += results.get(i).unwrap();
        }
    }

    //Part 2
    let mut sum2 = 0;
    for i in 0..results.len() {
        if check2(i as u128, &results, &numbers){
            sum2 += results.get(i).unwrap();
        }
    }

    println!("The solution to part 1 is:{}", sum1);
    println!("The solution to part 2 is:{}", sum2);
}
fn check(i : u128, results : &Vec<u128>, numbers: &Vec<Vec<u128>>) -> bool {
    let result = results[i as usize];
    let nums : Vec<u128> = numbers[i as usize].clone(); 

    let amount_permutations = 1 << (nums.len()-1);
    let mut permutations : Vec<Vec<char>> = vec![];
    for i in 0..amount_permutations {
        let mut operations : Vec<char> = vec![]; 
        for j in 0..nums.len()-1{
            if (i & (1 << j)) >> j == 1 {
                operations.push('*');
            }else{
                operations.push('+');
            }
        }
        permutations.push(operations);
    }
    for permutation in permutations {
        let mut result_ = nums[0];
        for i in 0..permutation.len() {
            if permutation[i] == '*' {
                result_ *= nums[i+1];
            }else{
                result_ += nums[i+1];
            }
        }
        if result == result_ {
            return true
        }
    }
    false
}

fn check2(i : u128, results : &Vec<u128>, numbers: &Vec<Vec<u128>>) -> bool {
    let result = results[i as usize];
    let nums : Vec<u128> = numbers[i as usize].clone(); 

    let amount_permutations = 3_u128.pow((nums.len()-1).try_into().unwrap());
    let mut permutations : Vec<Vec<char>> = vec![];
    for i in 0..amount_permutations {
        let mut operations: Vec<char> = Vec::new();
        let mut value = i;
        for _ in 0..nums.len()-1 {
            match value % 3 {
                0 => operations.push('|'),
                1 => operations.push('*'),
                _ => operations.push('+'),
            }
            value /= 3;
        }
        permutations.push(operations);
    }
    for permutation in permutations {
        let mut result_ = nums[0];
        for i in 0..permutation.len() {
            if permutation[i] == '*' {
                result_ *= nums[i+1];
            }else if permutation[i] == '+'{
                result_ += nums[i+1];
            }else{
                result_ = result_ * 10_u128.pow((nums[i+1]).to_string().len().try_into().unwrap()) + nums[i+1];
            }
        }
        if result == result_ {
            return true
        }
    }
    false
}