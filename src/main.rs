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
        [1,5,9,13],
        [2,6,10,14],
        [3,7,11,15],
        [4,8,12,16]
    ];

    println!("{}", game[0][0]);
    game::print_board_game(&mut game);
    println!("******************************");
    game::rotate_board_game(&mut game);
    game::print_board_game(&mut game);
}

