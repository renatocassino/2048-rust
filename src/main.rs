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

fn print_logo(rustbox: &RustBox, version: &str) {
    let default_color = Color::Default;
    let line_version = format!("                                                                 Version: {}", version);
    rustbox.print(4,1,rustbox::RB_BOLD, default_color, default_color, "    _______  _______  _   ___   _____     _______  _______  __   __  _______ ");
    rustbox.print(4,2,rustbox::RB_BOLD, default_color, default_color, "   |       ||  _    || | |   | |  _  |   |       ||   _   ||  |_|  ||       |");
    rustbox.print(4,3,rustbox::RB_BOLD, default_color, default_color, "   |____   || | |   || |_|   | | |_| |   |    ___||  |_|  ||       ||    ___|");
    rustbox.print(4,4,rustbox::RB_BOLD, default_color, default_color, "    ____|  || | |   ||       ||   _   |  |   | __ |       ||       ||   |___ ");
    rustbox.print(4,5,rustbox::RB_BOLD, default_color, default_color, "   | ______|| |_|   ||___    ||  | |  |  |   ||  ||       ||       ||    ___|");
    rustbox.print(4,6,rustbox::RB_BOLD, default_color, default_color, "   | |_____ |       |    |   ||  |_|  |  |   |_| ||   _   || ||_|| ||   |___ ");
    rustbox.print(4,7,rustbox::RB_BOLD, default_color, default_color, "   |_______||_______|    |___||_______|  |_______||__| |__||_|   |_||_______|");
    rustbox.print(4,8,rustbox::RB_BOLD, default_color, default_color, "                                                                  By Tacnoman");
    rustbox.print(4,9,rustbox::RB_BOLD, default_color, default_color, &line_version);
}

fn print_message(rustbox: &RustBox) {
    rustbox.print(8, 30, rustbox::RB_BOLD, Color::Default, Color::Default, "You can control the board with the keys w,a,s and d.");
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

    // Start game with two numbers
    game::add_number(&mut game);
    game::add_number(&mut game);

    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut winner = false;
    let mut success = false;
    loop {
        print_logo(&rustbox, &version);
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
                    _ => { continue; }
                }
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

    game::print_board_game(&rustbox, &game);

    loop {
        if winner {
            let message = " Congrats! You win the game!! Press Q to quit. ";
            rustbox.print(8, 28, rustbox::RB_BOLD, Color::Default, Color::Green, message);
        } else {
            let message = " You loose the game! Press Q to quit. ";
            rustbox.print(8, 28, rustbox::RB_BOLD, Color::Default, Color::Red, message);
        }
        rustbox.present();
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    _ => { continue; }
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }
}
