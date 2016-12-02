extern crate rand;

use std::io;
use self::rand::Rng;

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

fn print_number_with_pad(number: i32, last: bool) {
    if number < 10 {
        print!("   {}", number);
    } else if number < 100 {
        print!("  {}", number);
    } else if number < 1000 {
        print!(" {}", number);
    } else {
        print!("{}", number);
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
