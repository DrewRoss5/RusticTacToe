use std::io;
use game::game::{Board, Player};

mod game;

// asks the user to input the coordinates they'd like to place a mark on and returns the resulting usize array
fn get_coords() -> [usize; 2]{
    let mut coords: [usize; 2] = [0; 2];
    let mut tmp_str: String = String::from("");
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

// runs a single player's turn given their symbol 
fn take_turn(player: &Player, board: &mut Board){
    println!("Player {}, your turn", player.number);
    board.display();
    let mut valid_move = false;
    let mut coords: [usize; 2];
    while !valid_move{
        coords = get_coords();
        match board.update_space(coords[0], coords[1], player.symbol){
            Ok(_) => {valid_move = true;}
            Err(_) => {}
        } 
    }
}

fn main() {
    // create game objects
    let mut game_board = Board::new();
    let player1: Player = Player{ number: 1, symbol: "X" };
    let player2: Player = Player{ number: 2, symbol: "O" };
    while game_board.turns < 9 && game_board.check_winner() == " "{
        // run the current player's turn 
        if (game_board.turns % 2) == 0{
            take_turn(&player1, &mut game_board)
        }
        else{
            take_turn(&player2, &mut game_board)
        }
    }
    game_board.display();
    // determine the game's winner 
    match game_board.check_winner(){
        "X" => {println!("Player 1 wins!")}
        "O" => {println!("Player 2 wins!")}
        _ => {println!("Tie!")}
    }

}