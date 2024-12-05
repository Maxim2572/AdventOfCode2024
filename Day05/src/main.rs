use std::{fs::File, io::Read, vec};

#[derive(Debug)]
struct Rule {
    before: i32,
    after: i32,
}

impl Rule {
    fn matches(&self, nums: Vec<i32>) -> bool {
        let mut valid = true; 
        let mut before_found = false;
        let mut after_found = false;
        for &num in &nums {
            if num == self.before {
                before_found = true;
            }
            if num == self.after {
                after_found = true;
            }
            if after_found && !before_found{
                valid = false;
            }
        }
        if !before_found {
            valid = true;
        }
        valid
    }
    fn new(before : i32, after : i32 ) -> Self {
        Self{before, after}
    }
}

//function is AI-generated, because I suck at coding ):
fn sort_by_rules(mut nums: Vec<i32>, rules: &Vec<Rule>) -> Vec<i32> {
    nums.sort_by(|a, b| {
        for rule in rules {
            if *a == rule.before && *b == rule.after {
                return std::cmp::Ordering::Less;
            }
            if *a == rule.after && *b == rule.before {
                return std::cmp::Ordering::Greater;
            }
        }
        std::cmp::Ordering::Equal
    });
    nums
}

fn main() {
    let mut file = File::open("src/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut invalid_list : Vec<Vec<i32>> = vec![];

    let mut sum = 0;
    let mut rulebook : Vec<Rule> = vec![]; 
    for line in contents.lines(){
        match line.chars().nth(2).unwrap_or(' '){
            '|' => {
                let first = line.get(0..2).unwrap().parse::<i32>().unwrap();
                let second = line.get(3..5).unwrap().parse::<i32>().unwrap();
                let rule: Rule = Rule::new(first, second);
                rulebook.push(rule);
            }
            ',' => {
                let mut valid = true;
                let line_vec : Vec<i32>= line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
                for rule in &rulebook {
                    if !rule.matches(line_vec.clone()){
                        valid = false;
                    } 
                }
                if valid{
                    sum += line_vec.clone().get(line_vec.clone().len()/2).unwrap();
                }else{
                    invalid_list.push(line_vec);
                }
            }
            _ => {continue}
        }
    }

    //Part 2
    let mut sum2 = 0;
    for values in invalid_list{
        let sorted_values = sort_by_rules(values, &rulebook);
        sum2 += sorted_values[sorted_values.len() / 2];
    }
    
    println!("The solution to part 1 is {}", sum);
    println!("The solution to part 2 is {}", sum2);
}
