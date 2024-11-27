mod board;

fn main() {
    let board = board::Board::empty();
    println!("{}", format!("{board}"));
}
