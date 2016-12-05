extern crate rand;

use std::io;
use self::rand::Rng;

extern crate ansi_term;
use self::ansi_term::Colour::*;

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
    return !has_number_in_game(game, 0);
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
    while t >= 0 { // (t=x-1;t>=0;t--) {
        if game[t] != 0 { //(array[t]!=0) {
            if game[t] != game[x as usize] {
                // merge is not possible, take next position
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
    // we did not find a
    return x;
}


pub fn slide_array(line: &mut[i32; 4]) -> bool {
    let size = 4;
    let mut stop = 0;
    let mut score = 0;
    let mut success = false;

    for x in 0..size-1 {
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

fn get_colored_number(number: i32) -> String {
    if number == 0 {
        return White.paint(number.to_string()).to_string()
    } else if number == 2 {
        return Yellow.paint(number.to_string()).to_string();
    } else if number == 4 {
        return Yellow.bold().paint(number.to_string()).to_string();
    } else if number == 8 {
        return Cyan.paint(number.to_string()).to_string();
    } else if number == 16 {
        return Cyan.bold().paint(number.to_string()).to_string();
    } else if number == 32 {
        return Purple.paint(number.to_string()).to_string();
    } else if number == 64 {
        return Purple.bold().paint(number.to_string()).to_string();
    } else if number == 128 {
        return Green.paint(number.to_string()).to_string();
    } else if number == 256 {
        return Green.bold().paint(number.to_string()).to_string();
    } else if number == 512 {
        return White.bold().paint(number.to_string()).to_string();
    } else if number == 1024 {
        return Red.paint(number.to_string()).to_string();
    } else if number == 2048 {
        return Red.bold().paint(number.to_string()).to_string();
    }

    return number.to_string();
}

fn print_number_with_pad(number: i32, last: bool) {
    let color_number:String = get_colored_number(number);
    
    if number < 10 {
        print!("   {}", color_number);
    } else if number < 100 {
        print!("  {}", color_number);
    } else if number < 1000 {
        print!(" {}", color_number);
    } else {
        print!("{}", color_number);
    }

    if !last {
        print!(" | ");
    }
}

pub fn print_board_game(game: &[[i32; 4]; 4]) {
    for y in 0..4 {
        for x in 0..4 {
            print_number_with_pad(game[x][y], x == 3);
        }

        if y != 4 {
            println!("");
            println!("-------------------------");
        }
    }
}
