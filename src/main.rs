mod game;

fn main() {
    println!("Hello, world!");
    let mut b: game::Board = game::Board::new(8, 8);
    b.init_checkers();
    println!("{}", b.to_string());
}
