mod game;
use std::io;
extern crate rustbox;
use rustbox::{Color, RustBox, OutputMode};
use rustbox::Key;

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

fn print_logo(rustbox: &RustBox) {
    rustbox.print(1,1,rustbox::RB_BOLD, Color::White, Color::Black, "    _______  _______  _   ___   _____    _______  _______  __   __  _______ ");
    rustbox.print(1,2,rustbox::RB_BOLD, Color::White, Color::Black, "   |       ||  _    || | |   | |  _  |  |       ||   _   ||  |_|  ||       |");
    rustbox.print(1,3,rustbox::RB_BOLD, Color::White, Color::Black, "   |____   || | |   || |_|   | | |_| |  |    ___||  |_|  ||       ||    ___|");
    rustbox.print(1,4,rustbox::RB_BOLD, Color::White, Color::Black, "    ____|  || | |   ||       ||   _   | |   | __ |       ||       ||   |___ ");
    rustbox.print(1,5,rustbox::RB_BOLD, Color::White, Color::Black, "   | ______|| |_|   ||___    ||  | |  | |   ||  ||       ||       ||    ___|");
    rustbox.print(1,6,rustbox::RB_BOLD, Color::White, Color::Black, "   | |_____ |       |    |   ||  |_|  | |   |_| ||   _   || ||_|| ||   |___ ");
    rustbox.print(1,7,rustbox::RB_BOLD, Color::White, Color::Black, "   |_______||_______|    |___||_______| |_______||__| |__||_|   |_||_______|");
    rustbox.print(1,8,rustbox::RB_BOLD, Color::White, Color::Black, "                                                                 By Tacnoman");
    rustbox.print(1,9,rustbox::RB_BOLD, Color::White, Color::Black, "                                                                Version: 1.0");
}

fn print_message(rustbox: &RustBox) {
    rustbox.print(1, 30, rustbox::RB_BOLD, Color::White, Color::Black, "You can control the board with the keys w,a,s and d.");
}

fn main() {
    let version = "1.0";

    
    let mut game: [[i32; 4]; 4] =
        [
            [0,0,0,0], // Line is a column
            [0,0,0,0],
            [0,0,0,0],
            [0,0,0,0]
        ];
    
    game::add_number(&mut game);
    game::add_number(&mut game);

    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Hello, world!");
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black,
                  "Press 'q' to quit.");

    let mut winner = false;
    let mut success = false;
    loop {
        print_logo(&rustbox);
        print_message(&rustbox);

        game::print_board_game(&rustbox, &game);
        rustbox.present();
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    Key::Char('w') => { success = move_up(&mut game); }
                    Key::Char('a') => { success = move_left(&mut game); }
                    Key::Char('s') => { success = move_down(&mut game); }
                    Key::Char('d') => { success = move_right(&mut game); }
                    _ => { }
                }
                game::add_number(&mut game);
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }

        if !success {
            continue;
        }

        if game::is_winner_game(&mut game) {
            winner = true;
            break;
        }

        if game::is_looser_game(&mut game) {
            break;
        }

        game::add_number(&mut game);
    }

    //    let mut looser = false;
    //    let mut success = false;
    //    while true {
    //        let answer = ask_movement();
    //
    //        if answer.trim() == "w" {
    //            success = move_up(&mut game);
    //        } else if answer.trim() == "a" {
    //            success = move_left(&mut game);
    //        } else if answer.trim() == "d" {
    //            success = move_right(&mut game);
    //        } else if answer.trim() == "s" {
    //            success = move_down(&mut game);
    //        } else {
    //            continue;
    //        }
    //
    //        if !success {
    //            continue;
    //        }
    //
    //
    //    }
    //
    //    if looser {
    //        println!("Ops! You loose the game!");
    //    } else {
    //        println!("CONGRATULATIONS! You win the game");
    //    }
}

