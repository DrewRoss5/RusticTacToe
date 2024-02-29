use std::fmt::Error;



struct Board{
    rows: [[String; 3]; 3],
    turns: u8,
}

impl Board {
    fn new() -> Board{
        Board{rows:[[" ".to_string(), " ".to_string(), " ".to_string()], 
                    [" ".to_string(), " ".to_string(), " ".to_string()], 
                    [" ".to_string(), " ".to_string(), " ".to_string()]], turns: 0}
    }
    
    // prints out the tic-tac-toe board
    fn display(&self){
        println!("   1   2   3");
        for i in 0..3{
            println!("{}  {} | {} | {} ", i+1, self.rows[i][0], self.rows[i][1], self.rows[i][2]);
            if i != 2{
                println!("   ------------");
            }
        }
    }

    // updates the character at the provided row and column to the provided character, returns an error if the row and column already has been assigned
    fn update_space(&mut self, row: usize, col: usize, new_char: &str ) -> Result<(), std::fmt::Error>{
        if self.rows[row][col] == " "{
            self.rows[row][col] = new_char.to_string();
            self.turns += 1;
            Ok(())
        }
        else{
            Err(Error)
        }
    }

    // returns if there is a winner and the character they're playing as if so. If a " " is returned, there is no winner
    fn check_winner(&self) -> String{
        // check for a winner on each column and row
        for i in 0..3{
            if self.rows[i][0] == self.rows[i][1] && self.rows[i][1] == self.rows[i][2]{
                return self.rows[i][0].to_string();
            }
            else if self.rows[0][i] == self.rows[1][i] && self.rows[1][i] == self.rows[2][i]{
                return self.rows[0][i].to_string();
            }
        }
        // check for a winner on either of the diagonals
        if (self.rows[1][1] == self.rows[0][0] && self.rows[0][0] == self.rows[2][2]) || (self.rows[1][1] == self.rows[2][0] && self.rows[0][0] == self.rows[0][2]){
            return self.rows[1][1].to_string();
        }
        " ".to_string()
    }
    
}


fn main() {

}
