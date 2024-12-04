use std::{fs::File, io::Read, vec};

fn main() {
    let mut file = File::open("src/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    //Part 1

    fn count_xmas(input: String)-> usize{
        input.matches("XMAS").count() + input.matches("SAMX").count()
    }

    let width = contents.clone().lines().next().unwrap().len();
    //rotate string
    let mut str_rotated = String::from("");
    for i in 0..width{
        for line in contents.lines() {
            let mut line_iter = line.chars(); 
            for _ in 0..i{
                line_iter.next();
            }
            str_rotated.push(line_iter.next().unwrap());
        }
        str_rotated.push('\n');
    }
    let mut content_vec: Vec<Vec<u8>> = vec![];

    for line in contents.lines() {
        content_vec.push(line.chars().as_str().as_bytes().to_vec());
    }
    let mut diagonal = String::new();

    //bottom left covered
    for j in 0..width{
        for i in 0..(width-j){
            diagonal.push(content_vec[i+j][i] as char);
            if i+j >= 139{
                diagonal.push('\n');
            }
        }
    }
    //top left covered
    for j in 0..width{
        for i in 0..(width-j){
            diagonal.push(content_vec[width-1-(i+j)][i] as char);
            if i+j >= 139{
                diagonal.push('\n');
            }
        }
    }
    //top right covered
    for j in 1..width{
        for i in 0..(width-j){
            diagonal.push(content_vec[width-1-(i+j)][width-1-i] as char);
            if i+j >= 139{
                diagonal.push('\n');
            }
        }
    }
    //bottom right covered
    for j in 1..width{
        for i in 0..(width-j){
            diagonal.push(content_vec[i+j][width-1-i] as char);
            if i+j >= 139{
                diagonal.push('\n');
            }
        }
    }

    // Part 2
    let mut sum = 0;
    for i in 0..width{
        for j in 0..width{
            let mut valid = true;
            if i > 0 && j> 0 && i<139 && j<139 {
                if content_vec[i][j] == b'A' {
                    match content_vec[i-1][j-1] {
                        b'M' => {
                            if content_vec[i+1][j+1] != b'S'{
                                valid = false;
                            }
                        },
                        b'S' => {
                            if content_vec[i+1][j+1] != b'M'{
                                valid = false;
                            }
                        },
                        _ => valid = false,

                    }
                    match content_vec[i+1][j-1] {
                        b'M' => {
                            if content_vec[i-1][j+1] != b'S'{
                                valid = false;
                            }
                        },
                        b'S' => {
                            if content_vec[i-1][j+1] != b'M'{
                                valid = false;
                            }
                        },
                        _ => valid = false,

                    }
                }else{
                    valid = false;
                }
            }else{
                valid = false;
            }
            if valid {
                sum += 1;
            }
        }
    }
    println!("The solution to part 1 is: {}", count_xmas(contents)+count_xmas(str_rotated)+count_xmas(diagonal));
    println!("The solution to part 2 is: {}", sum);
}

