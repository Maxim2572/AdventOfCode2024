use std::{collections::{HashMap, HashSet}, fs::File, io::Read};

fn main() {
    let mut file = File::open("src/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut antennas : HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let width = contents.lines().next().unwrap().len() as i32;
    let length = contents.lines().count() as i32;
    for (i, line) in contents.lines().enumerate() {
        for (j, current_char) in line.chars().enumerate() {
            if current_char != '.' {
                antennas
                    .entry(current_char)
                    .or_insert_with(Vec::new)
                    .push((i, j));
            }
        }
    }

    let mut antinodes : HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (ch,positions) in antennas.clone() {
        if positions.len() == 1{
            continue;
        }
        for antenna in &positions {
            for new_antenna in &positions {
                if antenna == new_antenna {
                    continue
                }
                let new_x = 2 * antenna.1 as i32 - new_antenna.1 as i32;
                let new_y = 2 * antenna.0 as i32 - new_antenna.0 as i32;
                if new_x >= width || new_y >= length || new_x < 0 || new_y < 0{
                    continue;
                }
                antinodes.entry(ch).or_insert_with(Vec::new).push((new_y as usize, new_x as usize));
            }
        }
    }

    
    let sum1 = antinodes
    .values()
    .flat_map(|positions| positions.iter().copied())
    .collect::<HashSet<(usize, usize)>>()
    .len();

    //Part 2
    let mut antinodes : HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (ch,positions) in antennas {
        if positions.len() == 1{
            continue;
        }
        for antenna in &positions {
            for new_antenna in &positions {
                if antenna == new_antenna {
                    continue
                }
                for i in -width..width {
                    let new_x = (i+1) * antenna.1 as i32 - i * new_antenna.1 as i32;
                    let new_y = (i+1) * antenna.0 as i32 - i * new_antenna.0 as i32;
                    if new_x >= width || new_y >= length || new_x < 0 || new_y < 0{
                        continue;
                    }
                    antinodes.entry(ch).or_insert_with(Vec::new).push((new_y as usize, new_x as usize));
                }
            }
        }
    }

    let sum2 = antinodes
    .values()
    .flat_map(|positions| positions.iter().copied())
    .collect::<HashSet<(usize, usize)>>()
    .len();

    println!("The solution to part 1 is {}", sum1);
    println!("The solution to part 2 is {}", sum2);
}