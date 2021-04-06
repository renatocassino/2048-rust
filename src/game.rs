extern crate rand;

use self::rand::Rng;

extern crate ansi_term;

extern crate rustbox;
use self::rustbox::{Color, RustBox, Style};

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

pub fn get_empty_blocks(game: &mut[[i32; 4]; 4]) -> Vec<[usize; 2]> {
    // let list = game.iter().fold([], |list_empty, line| {
    //     list_empty.push(1);
    //     return list_empty;
    // });

    let mut list_empty = Vec::new();

    for y in 0..4 {
        for x in 0..4 {
            if game[x][y] == 0 {
                list_empty.push([x, y]);
            }
        }
    }

    return list_empty;
}

pub fn add_number(game: &mut[[i32; 4]; 4]) {
    let list_empty = get_empty_blocks(game);

    let secret_number = rand::thread_rng().gen_range(0..list_empty.len());
    let number_to_add = if (rand::thread_rng().gen_range(0..10) % 2) == 0 { 2 } else { 4 };

    let v1:usize = list_empty[secret_number][0];
    let v2:usize = list_empty[secret_number][1];
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

fn get_number_with_pad(number: i32) -> String {
    let formated_number: String;
    if number == 0 {
        formated_number = format!("     ");
    } else if number < 10 {
        formated_number = format!("  {}  ", number.to_string());
    } else if number < 100 {
        formated_number = format!("  {} ", number.to_string());
    } else if number < 1000 {
        formated_number = format!(" {} ", number.to_string());
    } else {
        formated_number = format!(" {}", number.to_string());
    }
    return formated_number;
}

fn get_color(value: i32) -> Color {
    if value == 2 || value == 4 || value == 16 || value == 64 || value == 2048 {
        return Color::White;
    } else if value == 8 || value == 512 {
        return Color::Red;
    } else if value == 32 {
        return Color::Black;
    } else if value == 128 || value == 256 {
        return Color::Yellow;
    }
    return Color::Magenta;
}

fn get_bg(value: i32) -> Color {
    if value == 0 {
        return Color::Black;
    } else if value == 2 {
        return Color::Cyan;
    } else if value == 4 || value == 256 {
        return Color::Magenta;
    } else if value == 8 || value == 1024 {
        return Color::White;
    } else if value == 16 {
        return Color::Green;
    } else if value == 32 || value == 512 {
        return Color::Yellow;
    } else if value == 128 || value == 2048 {
        return Color::Red;
    }
    return Color::Blue;
}

fn get_font_style(value: i32) -> Style {
    if value == 128 {
        return rustbox::RB_NORMAL;
    }
    return rustbox::RB_BOLD;
}

fn print_block(rustbox: &RustBox, x: usize, y: usize, value: i32) {
    let v = get_number_with_pad(value);
    let space = "     ";

    let position_x = (x*6) + 8;
    let position_y = 13+y*4;

    let color = get_color(value);
    let bg = get_bg(value);
    let style = get_font_style(value);
    rustbox.print(position_x, position_y-1, style, color, bg, &space);
    rustbox.print(position_x, position_y, style, color, bg, &v);
    rustbox.print(position_x, position_y+1, style, color, bg, &space);

}

pub fn print_board_game(rustbox: &RustBox, game: &[[i32; 4]; 4]) {
    for y in 0..4 {
        for x in 0..4 {
            let s = game[x][y];
            print_block(&rustbox, x, y, s);
        }
    }
}
