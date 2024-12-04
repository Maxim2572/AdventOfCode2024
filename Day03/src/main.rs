use core::str;
use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("src/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //Part 1 
    let mut product = 0;
    for str in contents.clone().into_bytes().windows(12){
        product += is_valid(str::from_utf8(str).unwrap());
    }

    //Part 2
    let mut product2 = 0;
    let mut valid = true;
    for str in contents.clone().into_bytes().windows(12){
        if str::from_utf8(str).unwrap().starts_with("don't()"){
            valid = false;
        }
        if str::from_utf8(str).unwrap().starts_with("do()"){
            valid = true;
        }
        if valid{
            product2 += is_valid(str::from_utf8(str).unwrap());
        }
    }

    println!("{product}");
    println!("{product2}")
}

fn is_valid(str: &str) -> i32{
    if !str.starts_with("mul("){
        return 0;
    }
    let mut str_iter = str.chars();
    str_iter.next();
    str_iter.next();
    str_iter.next();
    str_iter.next();

    let mut first_num = String::from("");
    if str_iter.clone().next().unwrap().is_numeric(){
        first_num.push(str_iter.next().unwrap());
        if str_iter.clone().next().unwrap().is_numeric(){
            first_num.push(str_iter.next().unwrap());
            if str_iter.clone().next().unwrap().is_numeric(){
                first_num.push(str_iter.next().unwrap());
            }
        }
    }else{
        return 0
    }
    if str_iter.next().unwrap() != ','{
        return 0;
    }
    let mut second_num = String::from("");
    if str_iter.clone().next().unwrap().is_numeric(){
        second_num.push(str_iter.next().unwrap());
        if str_iter.clone().next().unwrap().is_numeric(){
            second_num.push(str_iter.next().unwrap());
            if str_iter.clone().next().unwrap().is_numeric(){
                second_num.push(str_iter.next().unwrap());
            }
        }
    }else{
        return 0
    }
    if str_iter.next().unwrap() != ')'{
        return 0;
    }
    return first_num.parse::<i32>().unwrap() * second_num.parse::<i32>().unwrap()
}