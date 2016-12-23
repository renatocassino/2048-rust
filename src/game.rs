extern crate rand;

use std::io;
use self::rand::Rng;

extern crate ansi_term;
use self::ansi_term::Colour::*;
use self::ansi_term::Style;

extern crate rustbox;
use self::rustbox::{Color, RustBox, OutputMode};
use self::rustbox::Key;

pub fn rotate_board_game(game: &mut[[i32; 4]; 4]) {
    let n = 4;
    for i in 0..(4/2) {
        for j in i..n-i-1 {
            let aux = game[i][j];
            game[i][j] = game[j][n-i-1];
            game[j][n-i-1] = game[n-i-1][n-j-1];
            game[n-i-1][n-j-1] = game[n-j-1][i];
            game[n-j-1][i] = aux;
        }
    }
}

pub fn add_number(game: &mut[[i32; 4]; 4]) {
    let mut indexes: [[usize; 2]; 16] = [
        [0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0]
    ];
    let mut count = 0;

    for y in 0..4 {
        for x in 0..4 {
            if game[x][y] == 0 {
                indexes[count][0] = x;
                indexes[count][1] = y;
                count = count + 1;
            }
        }
    }

    let secret_number = rand::thread_rng().gen_range(0, count);
    let number_to_add = if (rand::thread_rng().gen_range(0, 10) % 2) == 0 { 2 } else { 4 };

    let v1:usize = indexes[secret_number][0];
    let v2:usize = indexes[secret_number][1];
    game[v1][v2] = number_to_add;
}

fn has_number_in_game(game: &[[i32;4]; 4], number:i32) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if game[x][y] == number {
                return true;
            }
        }
    }

    return false;
}

pub fn is_looser_game(game: &[[i32;4]; 4]) -> bool {
    let has_zero = has_number_in_game(game, 0);
    if has_zero {
        return false;
    }

    for y in 0..3 {
        for x in 0..3 {
            if game[x][y] == game[x+1][y] {
                return false;
            }
            if game[x][y] == game[x][y+1] {
                return false;
            }
        }
    }
    return true;
}

pub fn is_winner_game(game: &[[i32;4]; 4]) -> bool {
    return has_number_in_game(game, 2048);
}

fn find_target(game: &[i32; 4], x: i32, stop: i32) -> i32 {
    // if the position is already on the first, don't evaluate
    if x==0 {
        return x;
    }

    let mut t:usize = (x as usize) - 1;
    while t >= 0 {
        if game[t] != 0 {
            if game[t] != game[x as usize] {
                return (t as i32) + 1;
            }
            return t as i32;
        } else {
            // we should not slide further, return this one
            if t==stop as usize {
                return t as i32;
            }
        }
        t = t-1;
    }
    return x;
}


pub fn slide_array(line: &mut[i32; 4]) -> bool {
    let size = 4;
    let mut stop = 0;
    let mut score = 0;
    let mut success = false;

    for x in 0..size {
        if line[x] != 0 {
            let t = find_target(&line, x as i32, stop);
            if t != x as i32 {
                if line[t as usize] != 0 {
                    stop = t+1;
                }
                line[t as usize] += line[x];
                line[x] = 0;
                success = true;
            }
        }
    }
    return success;
}

fn get_colored_number(number_with_pad: String, number: i32) -> String {
    if number == 0 {
        return Style::new().on(Black).fg(White).paint(number_with_pad).to_string();
    } else if number == 2 {
        return Style::new().on(Yellow).fg(Black).paint(number_with_pad).to_string();
    } else if number == 4 {
        return Style::new().on(Cyan).fg(Black).paint(number_with_pad).to_string();
    } else if number == 8 {
        return Style::new().on(Purple).fg(Black).paint(number_with_pad).to_string();
    } else if number == 16 {
        return Style::new().on(Green).fg(Black).paint(number_with_pad).to_string();
    } else if number == 32 {
        return Style::new().on(Purple).fg(White).bold().paint(number_with_pad).to_string();
    } else if number == 64 {
        return Style::new().on(Green).fg(White).bold().paint(number_with_pad).to_string();
    } else if number == 128 {
        return Style::new().on(White).fg(Black).paint(number_with_pad).to_string();
    } else if number == 256 {
        return Style::new().on(Red).fg(Black).bold().paint(number_with_pad).to_string();
    } else if number == 512 {
        return Style::new().on(Cyan).fg(Yellow).paint(number_with_pad).to_string();
    } else if number == 1024 {
        return Style::new().on(Red).fg(Black).paint(number_with_pad).to_string();
    } else if number == 2048 {
        return Style::new().on(Red).fg(White).bold().paint(number_with_pad).to_string();
    }

    return number_with_pad;
}

fn get_number_with_pad(number: i32) -> String {
    let mut color: String;
    if number == 0 {
        color = format!("     ");
    } else if number < 10 {
        color = format!("  {}  ", number.to_string());
    } else if number < 100 {
        color = format!(" {}  ", number.to_string());
    } else if number < 1000 {
        color = format!(" {} ", number.to_string());
    } else {
        color = format!(" {}", number.to_string());
    }
    return color;
}

fn get_color(value: i32) -> Color {
    if value == 4 {
        return Color::Red;
    }
    return Color::White;
}

fn print_block(rustbox: &RustBox, x: usize, y: usize, value: i32) {
    let v = get_number_with_pad(value);
    let space = "     ";

    let position_x = x*6;
    let position_y = 13+y*4;

    let color = get_color(value);
//    bg = get_bg(value);
    rustbox.print(position_x, position_y-1, rustbox::RB_BOLD, color, Color::Black, &space);
    rustbox.print(position_x, position_y, rustbox::RB_BOLD, color, Color::Black, &v);
    rustbox.print(position_x, position_y+1, rustbox::RB_BOLD, color, Color::Black, &space);

}

pub fn print_board_game(rustbox: &RustBox, game: &[[i32; 4]; 4]) {
    for y in 0..4 {
        for x in 0..4 {
            let s = game[x][y];
            print_block(&rustbox,x,y,s);
        }
    }
}
