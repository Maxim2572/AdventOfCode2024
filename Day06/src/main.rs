use std::{fs::File, io::Read};
fn main() {
    //Part 1
    let mut file = File::open("src/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut pos_y : usize = 0;
    let mut position_guard = (0,0);
    let mut init_field : Vec<Vec<char>> = vec![];
    for line in contents.lines(){
        init_field.push(line.chars().collect());
        if line.find('^').unwrap_or(0) != 0{
            position_guard = (line.find('^').unwrap()+1, pos_y-1);
        }
        pos_y += 1;
    }
    
    let mut field = Field::new(init_field.clone(), position_guard);
    let mut in_field = true;
    println!("guard {} at {:?}" , init_field.clone()[position_guard.0][position_guard.1], position_guard);
    
    while in_field {
        in_field = field.advance();
    }
    let mut sum1 = 0;
    for vec in &field.field {
        for item in vec{
            if item == &'X'{
                sum1 += 1;
            }
        }
    }

    //Part 2
    let mut potential_solutions : Vec<Vec<Vec<char>>> = vec![];
    for i in 0..init_field.iter().nth(1).unwrap().len(){
        for j in 0..init_field.iter().len(){
            if init_field[i][j] == '.'{
                let mut cloned_vec = init_field.clone();
                cloned_vec[i][j] = '#';
                potential_solutions.push(cloned_vec);
            }
        }
    }  

    let mut sum2 = 0;
    let mut solutions_checked = 0;
    let len = potential_solutions.len();
    for solution in potential_solutions {
        solutions_checked += 1;
        println!("{}% done", (solutions_checked as f32)/(len as f32)*100.0);
        let mut field2 = Field::new(solution, position_guard);
        let mut in_field_2 = true;
        let mut step_count= 0;
        while in_field_2 {
            step_count += 1;
            let booleans = field2.modified_advance(); 
            in_field_2 = booleans.0;
            if booleans.1 {
                sum2 += 1;
                break;
            }
            if step_count > 100000 {
                sum2 += 1;
                break;
            }
        }
    }

    println!("The solution to part 1 is {}", sum1);
    println!("The solution to part 2 is {:?}", sum2);
}

#[derive(Debug, Clone)]
struct Field {
    field : Vec<Vec<char>>,
    pos_guard : (usize, usize),
}
impl Field {
    fn new(field: Vec<Vec<char>>, pos_guard : (usize, usize)) -> Self {
        Self{field, pos_guard}
    }
    fn advance(&mut self) -> bool {
        match self.field.clone().get(self.pos_guard.0 ){
            Some(_) => {
                match self.field.clone().get(self.pos_guard.0 ).unwrap().get(self.pos_guard.1 ) {
                    Some(char) => {
                        match char {
                            '<' => {
                                self.field[self.pos_guard.0 ][self.pos_guard.1 ] = 'X';
                                let mut next_pos = (0,0);
                                if self.pos_guard.1 as i32-1 >= 0{
                                    next_pos = (self.pos_guard.0, self.pos_guard.1 -1);
                                }else{
                                    return false;
                                }
                                match self.field[next_pos.0 ].get(next_pos.1 ){
                                    Some(ch) => {
                                        if *ch == '#' {
                                            self.field[self.pos_guard.0 ][self.pos_guard.1 ] = turn(*char);
                                        }else{
                                            self.field[next_pos.0 ][next_pos.1 ] = *char;
                                            self.pos_guard = (next_pos.0, next_pos.1);
                                        } 
                                    }
                                    None => {return false}
                                }
                                return true
                            }
                            '>' => {
                                self.field[self.pos_guard.0 ][self.pos_guard.1 ] = 'X';
                                let next_pos = (self.pos_guard.0, self.pos_guard.1 +1);  
                                match self.field[next_pos.0 ].get(next_pos.1 ){
                                    Some(ch) => {
                                        if *ch == '#' {
                                            self.field[self.pos_guard.0 ][self.pos_guard.1 ] = turn(*char);
                                        }else{
                                            self.field[next_pos.0 ][next_pos.1 ] = *char;
                                            self.pos_guard = (next_pos.0, next_pos.1);
                                        } 
                                    }
                                    None => {return false}
                                }
                                return true
                            }
                            '^' => {
                                self.field[self.pos_guard.0 ][self.pos_guard.1 ] = 'X';
                                let mut next_pos = (0,0);
                                if self.pos_guard.0 as i32-1 >= 0{
                                    next_pos = (self.pos_guard.0 -1, self.pos_guard.1);
                                }else{
                                    return false;
                                }                       
                                match self.field[next_pos.0 ].get(next_pos.1 ){
                                    Some(ch) => {
                                        if *ch == '#' {
                                            self.field[self.pos_guard.0 ][self.pos_guard.1 ] = turn(*char);
                                        }else{
                                            self.field[next_pos.0 ][next_pos.1 ] = *char;
                                            self.pos_guard = (next_pos.0, next_pos.1);
                                        } 
                                    }
                                    None => {return false}
                                }
                                return true
                            }
                            'v' => {
                                self.field[self.pos_guard.0 ][self.pos_guard.1 ] = 'X';
                                let next_pos = (self.pos_guard.0 +1, self.pos_guard.1);
                                match self.field[next_pos.0].get(next_pos.1 ){
                                    Some(ch) => {
                                        if *ch == '#' {
                                            self.field[self.pos_guard.0 ][self.pos_guard.1 ] = turn(*char);
                                        }else{
                                            self.field[next_pos.0 ][next_pos.1 ] = *char;
                                            self.pos_guard = (next_pos.0, next_pos.1);
                                        } 
                                    }
                                    None => {return false}
                                }
                                return true
                            }
                            _ =>  {
                                false
                            }
                        }
                    }
                    None => {false}
                }
            }
            None => {false}
        }
    }
    fn modified_advance(&mut self) -> (bool, bool) {
        let field_clone = self.field.clone();
        match field_clone.get(self.pos_guard.0 ){
            Some(_) => {
                match field_clone.get(self.pos_guard.0 ).unwrap().get(self.pos_guard.1 ) {
                    Some(char) => {
                        match char {
                            '<' => {
                                self.field[self.pos_guard.0 ][self.pos_guard.1 ] = *char;
                                let mut next_pos = (0,0);
                                if self.pos_guard.1 as i32-1 >= 0{
                                    next_pos = (self.pos_guard.0, self.pos_guard.1 -1);
                                }else{
                                    return (false, false);
                                }
                                match self.field[next_pos.0 ].get(next_pos.1 ){
                                    Some(ch) => {
                                        if self.field[next_pos.0][next_pos.1] == *char {
                                            return (true,true);
                                        }
                                        if *ch == '#' {
                                            self.field[self.pos_guard.0 ][self.pos_guard.1 ] = turn(*char);
                                        }else{
                                            self.field[next_pos.0 ][next_pos.1 ] = *char;
                                            self.pos_guard = (next_pos.0, next_pos.1);
                                        } 
                                    }
                                    None => {return (false, false)}
                                }
                                
                                return (true, false)
                            }
                            '>' => {
                                self.field[self.pos_guard.0 ][self.pos_guard.1 ] = *char;
                                let mut next_pos = (0,0);
                                if self.pos_guard.1 as i32+1 <130{
                                    next_pos = (self.pos_guard.0, self.pos_guard.1 +1);
                                }else{
                                    return (false, false);
                                } 
                                match self.field[next_pos.0 ].get(next_pos.1 ){
                                    Some(ch) => {
                                        if self.field[next_pos.0][next_pos.1] == *char {
                                            return (true,true);
                                        }
                                        if *ch == '#' {
                                            self.field[self.pos_guard.0 ][self.pos_guard.1 ] = turn(*char);
                                        }else{
                                            self.field[next_pos.0 ][next_pos.1 ] = *char;
                                            self.pos_guard = (next_pos.0, next_pos.1);
                                        } 
                                    }
                                    None => {return (false, false)}
                                }
                                return (true, false)
                            }
                            '^' => {
                                self.field[self.pos_guard.0 ][self.pos_guard.1 ] = *char;
                                let mut next_pos = (0,0);
                                if self.pos_guard.0 as i32-1 >= 0{
                                    next_pos = (self.pos_guard.0 -1, self.pos_guard.1);
                                }else{
                                    return (false, false);
                                }                       
                                match self.field[next_pos.0 ].get(next_pos.1 ){
                                    Some(ch) => {
                                        if self.field[next_pos.0][next_pos.1] == *char {
                                            return (true,true);
                                        }
                                        if *ch == '#' {
                                            self.field[self.pos_guard.0 ][self.pos_guard.1 ] = turn(*char);
                                        }else{
                                            self.field[next_pos.0 ][next_pos.1 ] = *char;
                                            self.pos_guard = (next_pos.0, next_pos.1);
                                        } 
                                    }
                                    None => {return (false, false)}
                                }
                                return (true, false)
                            }
                            'v' => {
                                self.field[self.pos_guard.0 ][self.pos_guard.1 ] = *char;
                                let mut next_pos = (0,0);
                                if self.pos_guard.0 as i32+1 < 130{
                                    next_pos = (self.pos_guard.0 +1, self.pos_guard.1);
                                }else{
                                    return (false, false);
                                }                                 match self.field[next_pos.0].get(next_pos.1 ){
                                    Some(ch) => {
                                        if self.field[next_pos.0][next_pos.1] == *char {
                                            return (true,true);
                                        }
                                        if *ch == '#' {
                                            self.field[self.pos_guard.0 ][self.pos_guard.1 ] = turn(*char);
                                        }else{
                                            self.field[next_pos.0 ][next_pos.1 ] = *char;
                                            self.pos_guard = (next_pos.0, next_pos.1);
                                        } 
                                    }
                                    None => {return (false, false)}
                                }
                                return (true, false)
                            }
                            _ =>  {
                                (false, false)
                            }
                        }
                    }
                    None => {(false, false)}
                }
            }
            None => {(false, false)}
        }
    }
}



fn turn(ch: char) -> char {
    match ch {
        '<' => '^',
        '>' => 'v',
        'v' => '<',
        '^' => '>',
        _ => '\0',
    }
}