mod game;
use std::io;

fn ask_movement() -> String {
    let mut guess = String::new();
    println!("Set a move position [w,a,s,d]: ");

    io::stdin().read_line(&mut guess);
    return guess;
}

fn main() {
    //    println!("Guess the number!");
    //    let secret_number = rand::thread_rng().gen_range(1, 101);
    //    println!("The secret number is: {}", secret_number);
    //    println!("Please input your guess.");

    let mut game: [[i32; 4]; 4] =
        [
            [0,0,0,0],
            [0,0,0,0],
            [0,0,0,0],
            [0,0,0,0]
        ];

    println!("{}", game[0][0]);

    game::add_number(&mut game);
    game::add_number(&mut game);
    game::print_board_game(&game);

    while true {
        let answer = ask_movement();
        game::print_board_game(&game);

        break;
    }
}

