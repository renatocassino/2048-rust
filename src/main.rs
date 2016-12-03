mod game;
use std::io;

fn ask_movement() -> String {
    let mut guess = String::new();
    println!("Set a move position [w,a,s,d]: ");

    io::stdin().read_line(&mut guess);
    return guess;
}

fn main() {
    let mut game: [[i32; 4]; 4] =
        [
            [0,0,0,0], // Line, not column
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

        if game::is_winner_game(&game) {
            break;
        }
    }

    if looser {
        println!("Ops! You loose the game!");
    } else {
        println!("CONGRATULATIONS! You win the game");
    }
}

