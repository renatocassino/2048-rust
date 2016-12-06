mod game;
use std::io;

fn ask_movement() -> String {
    let mut guess = String::new();
    println!("Set a move position [w,a,s,d]: ");

    io::stdin().read_line(&mut guess);
    return guess;
}

fn move_up(game: &mut[[i32; 4]; 4]) -> bool {
    let mut success = false;
    for x in 0..4 {
        success |= game::slide_array(&mut game[x]);
    }
    return success;
}

fn move_left(mut game: &mut[[i32; 4]; 4]) -> bool {
    game::rotate_board_game(&mut game);
    let success = move_up(&mut game);
    game::rotate_board_game(&mut game);
    game::rotate_board_game(&mut game);
    game::rotate_board_game(&mut game);
    return success;
}

fn move_right(mut game: &mut[[i32; 4]; 4]) -> bool {
    game::rotate_board_game(&mut game);
    game::rotate_board_game(&mut game);
    game::rotate_board_game(&mut game);
    let success = move_up(&mut game);
    game::rotate_board_game(&mut game);
    return success;    
}

fn move_down(mut game: &mut[[i32; 4]; 4]) -> bool {
    game::rotate_board_game(&mut game);
    game::rotate_board_game(&mut game);
    let success = move_up(&mut game);
    game::rotate_board_game(&mut game);
    game::rotate_board_game(&mut game);
    return success;
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
    let mut success = false;
    while true {
        let answer = ask_movement();

        if answer.trim() == "w" {
            success = move_up(&mut game);
        } else if answer.trim() == "a" {
            success = move_left(&mut game);
        } else if answer.trim() == "d" {
            success = move_right(&mut game);
        } else if answer.trim() == "s" {
            success = move_down(&mut game);
        } else {
            continue;
        }

        if !success {
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

