extern crate rand;

mod game;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("failed to read line");
    println!("You guessed: {}", guess);
    
    let mut game: [[i32; 4]; 4] =
    [
        [1,2,3,4],
        [5,6,7,8],
        [9,10,11,12],
        [13,14,15,16]
    ];
    println!("{}", game[0][0]);
    game::print_board_game(&mut game);
    
}

