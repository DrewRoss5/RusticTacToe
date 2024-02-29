use std::io;
use game::game::Board;

mod game;

// asks the user to input the coordinates they'd like to place a mark on and returns the resulting usize array
fn get_coords() -> [usize; 2]{
    let mut coords: [usize; 2] = [0; 2];
    let mut tmp_str: String = String::from("");
    let mut tmp_i8: i8;
    let mut valid = false;
    // get the row
    while !valid{
        println!("What row would you like to place your mark on?");
        io::stdin().read_line(&mut tmp_str).unwrap();
        tmp_str = tmp_str.trim().to_string();
        match str::parse::<i8>(tmp_str.as_str()){
            Ok(val) => {

                if val < 1 || val > 3{
                    println!("Please provide a value between one and three");
                }
                else{
                    coords[0] = (val - 1) as usize;
                    valid = true;
                }
                tmp_str.clear();
                valid = true;
            }
            Err(_) => {println!("\"{}\" Please provide an integer between one and three", tmp_str);}
        }
    }
    valid = false;
    // parse the column
    while !valid{
        println!("What column would you like to place your mark on?");
        io::stdin().read_line(&mut tmp_str).unwrap();
        tmp_str = tmp_str.trim().to_string();
        match str::parse::<usize>(tmp_str.as_str()){
            Ok(val) => {
                if val < 1 || val > 3{
                    println!("Please provide a value between one and three!");
                }
                else{
                    coords[1] = (val - 1) as usize;
                    valid = true;
                }
            }
            Err(_) => {println!("Please provide an integer between one and three");}
        }
    }
    coords
    
}

fn main() {
    
}
