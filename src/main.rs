use game::game::Board;

mod game;

fn main() {
    let board = Board::new();
    board.display();
}