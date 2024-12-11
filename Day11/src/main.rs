use std::{fs::File, io::Read, time};

fn main() {
    let mut file = File::open("src/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // let mut stones : Vec<usize> = contents.split_whitespace().map(|f| f.parse::<usize>().unwrap()).collect();

    // for i in 0..25 {
    //     stones = blink(&mut stones);
    //     println!("At step: {}", i);
    // }

    let mut stones2 : Vec<usize> = contents.split_whitespace().map(|f| f.parse::<usize>().unwrap()).collect();
    let mut results : String = String::new();
    for i in 0..50 {
        let start = time::Instant::now();
        stones2 = replace_0s(&mut stones2);
        stones2 = split_nums_and_multiply_by_2024(&mut stones2);
        println!("At step: {} in {:?}", i, start.elapsed());
        results.push_str(format!("{},{:?}\n", i, start.elapsed().as_nanos()).as_str());
    }

    // println!("The solution to part 1 is: {}", stones.len());
    println!("{results}");
    println!("The solution to part 2 is: {}", stones2.len());
}
// fn blink(stones: &mut Vec<usize>) -> Vec<usize>{
//     let mut i = 0; 
//     while i < stones.len() {
//         if stones[i] == 0 {
//             stones[i] = 1;
//         } else if stones[i].to_string().len() % 2 == 0 {
//             let stone_string = stones[i].to_string();
//             let split_string = stone_string.split_at(stone_string.len() / 2);
//             stones.remove(i);
//             stones.insert(i, split_string.0.parse().unwrap());
//             stones.insert(i + 1, split_string.1.parse().unwrap());
//             i += 1; 
//         } else {
//             stones[i] = 2024 * stones[i];
//         }
//         i += 1; 
//     }
//     stones.to_vec()
// }
fn replace_0s(stones: &mut Vec<usize>) -> Vec<usize>{
    stones
        .into_iter()
        .map(|f|{
            if *f == 0{
                1
            }else{
                *f
            }
        })
    .collect()
}
fn split_nums_and_multiply_by_2024(stones: &Vec<usize>) -> Vec<usize> {
    let mut result = Vec::new();
    
    for &f in stones {
        let mut n = f;
        let mut digits = 0;
        while n > 0 {
            n /= 10;
            digits += 1;
        }

        if digits % 2 == 0 {
            let half_len = 10_usize.pow((digits / 2) as u32);
            let left = f / half_len;
            let right = f % half_len;
            result.push(left);
            result.push(right);
        } else {
            result.push(f * 2024);
        }
    }
    
    result
}
