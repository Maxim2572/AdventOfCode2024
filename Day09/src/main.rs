use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("src/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut disk : Vec<Option<usize>> = vec![];

    let mut content_iter = contents.chars();
    for i in 0..contents.len(){
        if content_iter.clone().next().is_none(){
            break;
        }
        for _ in 0..content_iter.next().unwrap().to_string().parse::<usize>().unwrap() {
            disk.push(Some(i));
        }
        if content_iter.clone().next().is_none(){
            break;
        }
        for _ in 0..content_iter.next().unwrap().to_string().parse::<usize>().unwrap() {
            disk.push(None);
        }
    }
    let mut disk_2 = disk.clone();
    //Part 1
    for (i, chunk) in disk.clone().iter().enumerate(){
        if chunk.is_some(){
            continue
        }else{
            let mut cloned_disk = disk.clone();
            let mut last_item = cloned_disk.last().unwrap();
            disk.truncate(disk.len()-1);
            while last_item.is_none(){            
                cloned_disk = disk.clone();
                last_item = cloned_disk.last().unwrap();
                disk.truncate(disk.len()-1);
            }
            if disk.get(i).is_none(){
                disk.push(*last_item);
                break;
            }
            disk.remove(i);
            disk.insert(i, *last_item);
        }
    }
    let mut sum1 = 0;
    for (i, chunk) in disk.clone().iter().enumerate() {
        sum1 += i*(chunk.unwrap());
    }

    //Part 2


    println!("The solution to part 1 is: {}", sum1);
}