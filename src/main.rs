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

    let mut looser = false;
    while true {
        let answer = ask_movement();

        if answer.trim() == "w" {
            println!("Go Up!");
        } else if answer.trim() == "a" {
            println!("Go Left");
        } else if answer.trim() == "d" {
            println!("Go Right");
        } else if answer.trim() == "s" {
            println!("GO DOWN");
        } else {
            continue;
        }

        game::add_number(&mut game);
        game::print_board_game(&game);
        if game::is_looser_game(&game) {
            looser = true;
            break;
        }
    }

    if looser {
        println!("Ops! You loose the game!");
    } else {
        println!("CONGRATULATIONS! You win the game");
    }
}

